use crate::error::SkillTreeUrlError;
use std::str::FromStr;

// This should be an enum with different versions
#[derive(Clone)]
pub struct SkillTreeUrl {
    pub class: u8,
    pub ascendancy: u8,
    pub nodes: Vec<u16>,
}

impl std::fmt::Debug for SkillTreeUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SkillTreeUrl")
            .field("class", &self.class)
            .field("ascendancy", &self.ascendancy)
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SkillTreeUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl FromStr for SkillTreeUrl {
    type Err = SkillTreeUrlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: maybe parse an actual poe url here?
        let data =
            base64::decode_config(s, base64::URL_SAFE).map_err(|_| SkillTreeUrlError::Decode)?;
        if data.len() < 6 {
            return Err(SkillTreeUrlError::Eof);
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
            _ => Err(SkillTreeUrlError::UnknownVersion(version)),
        }
    }
}

fn read_u16s(data: &[u8], start: usize, amount: usize) -> Result<Vec<u16>, SkillTreeUrlError> {
    if data.len() < start + amount * 2 {
        return Err(SkillTreeUrlError::Eof);
    }

    let mut result = Vec::with_capacity(amount);

    for index in 0..amount {
        let index = start + index * 2;
        let value = (data[index] as u16) << 8 | data[index + 1] as u16;
        result.push(value);
    }

    Ok(result)
}
