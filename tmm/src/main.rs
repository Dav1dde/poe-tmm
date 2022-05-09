const S: &str = "AAAABgMDf1wUCPTxbFnzvOqbimdxwGYmlSL074gN4q0KM2wEs1VLjDa4XdSNGGXYw1Su18_YJH_Gl_Qs8Ro4gpsEBy6Uxoo0S5BVocfjakcGwWv3wVF0ffF90om3ppmTJ3F5Sp8Wb-w4zRbfiivA0NAWv75P8NVb1RcvL12ApJE3VcYw-FBC8B9xhZeF2rlIjlZITLNmukmxNPeJvCYpj_o2xayY2RNo8peVEFEfGDpYUfsdFK7_OtgkqmHiLOJqQ49GY1-qf_rSoOZwu1XWEmmIb2aeBx5JUZUuRv5eAxqBJy8fAoPbaxeOE2wL7Bgvb6IAZOf5N1M1NukgBm0ZIbARlqcIAAt3Jq0Khk-4XTW91I3uqdjDGEgs8UYWNEu6GqHH-3198YXCL13fW6p__fyIbw==";

fn main() -> anyhow::Result<()> {
    let stu = S.parse::<tmm::SkillTreeUrl>()?;

    let options = tmm::Options {
        nodes: stu.nodes,
        ..Default::default()
    };
    let tree = tmm::render_svg(tmm::Version::V3_17, options);
    std::fs::write("/tmp/t.svg", tree).unwrap();

    Ok(())
}
