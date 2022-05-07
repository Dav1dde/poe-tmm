use std::str::FromStr;

// This should be an enum with different versions
pub struct SkillTreeUrl {
    pub class: u8,
    pub ascendancy: u8,
    pub nodes: Vec<u16>,
}

impl FromStr for SkillTreeUrl {
    // TODO this shouldnt be anyhow
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: maybe parse an actual poe url here?
        let data = base64::decode_config(s, base64::URL_SAFE)?;
        if data.len() < 6 {
            anyhow::bail!("invalid skill tree url");
        }

        let version = (data[0] as u32) << 24
            | (data[1] as u32) << 16
            | (data[2] as u32) << 8
            | data[3] as u32;
        let class = data[4];
        let ascendancy = data[5];

        match version {
            4 => {
                // let fullscreen = data[6];
                let nodes = read_u16s(&data, 7, (data.len() - 7) / 2)?;
                Ok(SkillTreeUrl {
                    class,
                    ascendancy,
                    nodes,
                })
            }
            5 | 6 => {
                let amount = data[6] as usize;
                let nodes = read_u16s(&data, 7, amount)?;
                Ok(SkillTreeUrl {
                    class,
                    ascendancy,
                    nodes,
                })
            }
            _ => {
                anyhow::bail!("invalid version")
            }
        }
    }
}

fn read_u16s(data: &[u8], start: usize, amount: usize) -> anyhow::Result<Vec<u16>> {
    if data.len() < start + amount * 2 {
        anyhow::bail!("not enough data to read {} nodes", amount);
    }

    let mut result = Vec::with_capacity(amount);

    for index in 0..amount {
        let index = start + index * 2;
        let value = (data[index] as u16) << 8 | data[index + 1] as u16;
        result.push(value);
    }

    Ok(result)
}
