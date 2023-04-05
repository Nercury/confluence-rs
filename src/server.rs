/**
 * ServerInfo
 * Note: Version 1.0.3 of Confluence would be major-version: 1, minor-version: 0, patch-level: 3. Version 2.0 would have a patch-level of 0, even if it's not visible in the version number.
 */
#[derive(Debug)]
pub struct ServerInfo {
    pub major_version: i32, // the major version number of the Confluence instance
    pub minor_version: i32, // the minor version number of the Confluence instance
    pub patch_level: i32, // the patch-level of the Confluence instance
    pub build_id: String, // the build ID of the Confluence instance (usually a number)
    pub development_build: bool, // Whether the build is a developer-only release or not
    pub base_url: String, // The base URL for the confluence instance
}
