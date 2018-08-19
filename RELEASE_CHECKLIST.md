# Release Checklist

Copy this file into your PR to make a new release!

- [ ] Bump the version in `Cargo.toml`
- [ ] Update the example source code embedded in the website
- [ ] Update all the WASM binary examples for the website
- [ ] Verify, on each major platform, that all examples work correctly
- [ ] Update the html_root_url in `src/lib.rs`
- [ ] Ensure *all* dependencies are up-to-date
- [ ] Ensure there are no bugs this release *opens* (not fixing a bug is acceptable, releasing a version with new known bugs is not)