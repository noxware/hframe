pub const MASK_TEMPLATE: &str = r#"
<svg id="{id}-svg" class="hframe-mask-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}">
  <defs>
    <mask id="{id}-mask" x="0" y="0" width="{width}" height="{height}">
      <rect x="0" y="0" width="{width}" height="{height}" fill="white" />
      {holes}      
    </mask>
  </defs>
</svg>
"#;

pub const HOLE_TEMPLATE: &str =
    r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="5" fill="black" />"#;
