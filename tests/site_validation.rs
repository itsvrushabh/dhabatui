use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[test]
fn test_site_structure_and_assets() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let site_dir = manifest_dir.join("site");
    if !site_dir.exists() {
        // Packaged crate where site/ is excluded per Cargo.toml
        return;
    }
    let index_path = site_dir.join("index.html");
    let styles_path = site_dir.join("styles.css");
    let license_path = manifest_dir.join("LICENSE");

    assert!(index_path.exists(), "site/index.html must exist");
    assert!(styles_path.exists(), "site/styles.css must exist");
    assert!(license_path.exists(), "LICENSE file must exist");

    // Verify SVG assets equality between assets/ and site/assets/
    let assets = ["cashier_flow.svg", "screenshot.svg"];
    for asset in &assets {
        let root_asset = manifest_dir.join("assets").join(asset);
        let site_asset = site_dir.join("assets").join(asset);

        assert!(
            root_asset.exists(),
            "assets/{asset} must exist in root assets"
        );
        assert!(
            site_asset.exists(),
            "site/assets/{asset} must exist in site assets"
        );

        let root_bytes = fs::read(&root_asset).unwrap();
        let site_bytes = fs::read(&site_asset).unwrap();
        assert_eq!(
            root_bytes, site_bytes,
            "assets/{asset} and site/assets/{asset} must be identical"
        );
    }

    // Verify HTML links, images, and anchors
    let html = fs::read_to_string(&index_path).unwrap();

    // Collect all element IDs defined in index.html
    let mut ids = HashSet::new();
    for part in html.split("id=\"").skip(1) {
        if let Some(id) = part.split('"').next() {
            ids.insert(id.to_string());
        }
    }

    // Verify all internal href targets
    for part in html.split("href=\"").skip(1) {
        if let Some(href) = part.split('"').next() {
            if let Some(target_id) = href.strip_prefix('#') {
                assert!(
                    ids.contains(target_id),
                    "Anchor link #{target_id} has no matching element id in site/index.html"
                );
            } else if !href.starts_with("http://")
                && !href.starts_with("https://")
                && !href.starts_with("data:")
                && !href.starts_with("mailto:")
            {
                let file_path = site_dir.join(href);
                assert!(
                    file_path.exists(),
                    "Local href target '{href}' does not exist in site directory"
                );
            }
        }
    }

    // Verify all local img/script src targets
    for part in html.split("src=\"").skip(1) {
        if let Some(src) = part.split('"').next() {
            if !src.starts_with("http://")
                && !src.starts_with("https://")
                && !src.starts_with("data:")
            {
                let file_path = site_dir.join(src);
                assert!(
                    file_path.exists(),
                    "Local src target '{src}' does not exist in site directory"
                );
            }
        }
    }
}
