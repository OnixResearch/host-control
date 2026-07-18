let NonEmpty = std.contract.from_predicate (fun value => std.is_string value && value != "") in
let Release = {
  schema | NonEmpty,
  repository | NonEmpty,
  package | NonEmpty,
  license | NonEmpty,
  required_checks | Array NonEmpty,
  non_claims | Array NonEmpty,
} in
{
  release | Release = {
    schema = "host-control.release-profile.v1",
    repository = "ssh://git@github.com/OnixResearch/host-control.git",
    package = "host-control-core",
    license = "MIT OR Apache-2.0",
    required_checks = ["cargo-test", "cargo-clippy", "wasm32-unknown-unknown", "core-purity", "consumer-conformance"],
    non_claims = ["not-daemon-correctness", "not-transport-security", "not-pty-preservation", "not-policy-authorization", "not-product-readiness"],
  },
}
