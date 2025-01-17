use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct EndeavourOSTarget {
    /// Fetch list of mirrors timeout in milliseconds
    #[structopt(long = "fetch-mirrors-timeout", default_value = "15000")]
    pub fetch_mirrors_timeout: u64,

    /// Max time to fetch mirror version
    #[structopt(long = "version-mirror-timeout", default_value = "3000")]
    pub version_mirror_timeout: u64,

    /// Max number of concurrent requests to fetch mirror versions
    #[structopt(long = "version-mirror-concurrency", default_value = "40")]
    pub version_mirrors_concurrency: usize,

    /// Either url or path to EndeavourOS mirror list file
    #[structopt(
        long = "mirror-list-file",
        default_value = "https://raw.githubusercontent.com/endeavouros-team/PKGBUILDS/master/endeavouros-mirrorlist/endeavouros-mirrorlist",
        verbatim_doc_comment
    )]
    pub mirror_list_file: String,

    /// Path to be joined to a mirror url and used for speed testing
    ///   the file should be big enough to allow for testing high
    ///   speed connections
    #[structopt(
        long = "path-to-test",
        default_value = "endeavouros/x86_64/endeavouros.files",
        verbatim_doc_comment
    )]
    pub path_to_test: String,

    /// comment prefix to use when outputting
    #[structopt(long = "comment-prefix", default_value = "# ")]
    pub comment_prefix: String,
}
