use std::path::PathBuf;

use futures::future;
use walkdir::WalkDir;

use crate::{
    app::App,
    nuget::client::Package,
    types::{PackageRef, Project},
};

pub enum ProjectFile {
    Solution(PathBuf),
    Project(PathBuf),
}

impl App {
    pub async fn get_packages_from_projects(&self) -> Vec<Package> {
        let tasks = self.projects.iter().map(|p| async move {
            return self
                .client
                .get_packages(
                    p.package_refs
                        .iter()
                        .map(|r| r.package_id.clone())
                        .collect(),
                )
                .await;
        });

        let package_results = future::join_all(tasks).await;

        package_results
            .into_iter()
            .filter_map(|p| match p {
                Ok(packages) => Some(packages),
                _ => None,
            })
            .flatten()
            .collect()
    }
}

pub fn get_project_packages() -> Vec<Project> {
    let project_files = find_project_files(".");

    let projects: Vec<ProjectFile> = project_files
        .into_iter()
        .filter(|f| matches!(f, ProjectFile::Project(_)))
        .collect();

    // TODO: Handle .snl project. Currently we just ignore it.
    // TODO: Display errors if parsing of any project fails.
    let unloaded_nuget_packages: Vec<Project> = projects
        .into_iter()
        .filter_map(|f| match f {
            ProjectFile::Project(path) => {
                let Ok(project) = parse_packages(&path) else {
                    return None;
                };

                println!("{}", project.project_name);

                Some(project)
            }
            _ => None,
        })
        .collect();

    return unloaded_nuget_packages;
}

fn parse_packages(path: &PathBuf) -> anyhow::Result<Project> {
    let content = std::fs::read_to_string(path)?;
    let document = roxmltree::Document::parse(&content)?;

    let packages = document
        .descendants()
        .filter(|n| n.has_tag_name("PackageReference"))
        .filter_map(|n| {
            let name = n.attribute("Include")?;
            let version = n.attribute("Version")?;
            Some(PackageRef {
                package_id: name.to_string(),
                version: version.to_string(),
            })
        })
        .collect();

    Ok(Project {
        project_name: path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string(),
        project_path: path.clone(),
        package_refs: packages,
    })
}

fn find_project_files(root: &str) -> Vec<ProjectFile> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "sln" && ext == "csproj")
                .unwrap_or(false)
        })
        .map(|e| {
            let path = e.path().to_path_buf();
            if e.path().extension().and_then(|ext| ext.to_str()) == Some("sln") {
                ProjectFile::Solution(path)
            } else {
                ProjectFile::Project(path)
            }
        })
        .collect()
}
