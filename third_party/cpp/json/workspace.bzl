
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repo():
    JSON_VERSION = "3.10.4"
    JSON_SHA256 = "62c585468054e2d8e7c2759c0d990fd339d13be988577699366fe195162d16cb"

    http_archive(
        name = "com_github_nlohmann_json",
        sha256 = JSON_SHA256,
        urls = [
            "https://github.com/nlohmann/json/releases/download/v{version}/include.zip".format(version = JSON_VERSION),
        ],
        build_file = "//third_party/cpp/json:json.BUILD",
    )
