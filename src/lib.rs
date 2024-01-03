use alpm::{Package, SigLevel};
use anyhow::{anyhow, Result};
use error::PacinfoError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub mod args;
pub mod error;

const ROOT_DIR: &str = "/";
const DB_PATH: &str = "/var/lib/pacman/";

#[derive(Serialize, Deserialize)]
struct SyncDBPackage {
    name: String,
    version: String,
    description: String,
    architecture: String,
    url: String,
    packager: String,
    builddate: i64,
    licenses: Vec<String>,
    provides: Vec<String>,
    replaces: Vec<String>,
    depends: Vec<String>,
    makedepends: Vec<String>,
    checkdepends: Vec<String>,
    optdepends: Vec<String>,
    required_by: HashSet<String>,
}

/// Retrieve a HashMap of all reverse dependencies.
fn get_reverse_deps_map(pacman: &alpm::Alpm) -> HashMap<String, HashSet<String>> {
    let mut reverse_deps: HashMap<String, HashSet<String>> = HashMap::new();
    let dbs = pacman.syncdbs();

    for db in dbs {
        for pkg in db.pkgs() {
            for dep in pkg.depends() {
                reverse_deps
                    .entry(dep.name().to_string())
                    .and_modify(|e| {
                        e.insert(pkg.name().to_string());
                    })
                    .or_insert_with(|| {
                        let mut modify = HashSet::new();
                        modify.insert(pkg.name().to_string());
                        modify
                    });
            }

            for dep in pkg.makedepends() {
                reverse_deps
                    .entry(dep.name().to_string())
                    .and_modify(|e| {
                        e.insert(pkg.name().to_string());
                    })
                    .or_insert_with(|| {
                        let mut modify = HashSet::new();
                        modify.insert(pkg.name().to_string());
                        modify
                    });
            }

            for dep in pkg.checkdepends() {
                reverse_deps
                    .entry(dep.name().to_string())
                    .and_modify(|e| {
                        e.insert(pkg.name().to_string());
                    })
                    .or_insert_with(|| {
                        let mut modify = HashSet::new();
                        modify.insert(pkg.name().to_string());
                        modify
                    });
            }
        }
    }

    reverse_deps
}

/// Attempt to find any match of a package in the syncdb.
fn find_package_anywhere<'a>(pkgname: &str, pacman: &'a alpm::Alpm) -> Result<Package<'a>> {
    let dbs = pacman.syncdbs();
    for db in dbs {
        if let Ok(pkg) = db.pkg(pkgname) {
            return Ok(pkg);
        }
    }
    Err(anyhow!(PacinfoError::PackageNotFound))
}

fn jsonify_package<'a>(
    package: &'a Package,
    reverse_deps_map: &HashMap<String, HashSet<String>>,
) -> Result<SyncDBPackage> {
    let package = SyncDBPackage {
        name: package.name().into(),
        version: package.version().to_string(),
        description: package.desc().unwrap_or_default().to_string(),
        architecture: package.arch().unwrap_or_default().to_string(),
        url: package.url().unwrap_or_default().to_string(),
        packager: package.packager().unwrap_or_default().to_string(),
        builddate: package.build_date(),
        licenses: package
            .licenses()
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
        provides: package
            .provides()
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
        replaces: package
            .replaces()
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
        depends: package
            .depends()
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
        makedepends: package
            .makedepends()
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
        checkdepends: package
            .checkdepends()
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
        optdepends: package
            .optdepends()
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
        required_by: reverse_deps_map
            .get(package.name())
            .cloned()
            .unwrap_or([].into()),
    };

    Ok(package)
}

/// Run pacinfo, returning the json of package(s).
pub fn run(pkgnames: Vec<String>, dbpath: Option<String>, repos: Vec<String>) -> Result<String> {
    // TODO: alpm_utils? https://docs.rs/alpm-utils/latest/alpm_utils/fn.configure_alpm.html
    let pacman = match dbpath {
        Some(path) => alpm::Alpm::new(ROOT_DIR, &path),
        None => alpm::Alpm::new(ROOT_DIR, DB_PATH),
    }
    .map_err(PacinfoError::PacmanDbInit)?;

    for repo in repos {
        let _repo = pacman.register_syncdb(repo, SigLevel::DATABASE_OPTIONAL);
    }

    // alpm_pkg_compute_requiredby does not handle check/make depends so we build our own map
    let reverse_deps_map = get_reverse_deps_map(&pacman);

    let mut packages = Vec::new();
    for pkg in &pkgnames {
        let repopkg = find_package_anywhere(pkg, &pacman)?;
        packages.push(jsonify_package(&repopkg, &reverse_deps_map)?);
    }

    Ok(serde_json::to_string(&packages)?)
}
