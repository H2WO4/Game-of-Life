use lazy_regex::{regex, Lazy, Regex};

pub struct Rules {
    pub survive: Box<[u8]>,
    pub birth:   Box<[u8]>,

    pub birth_arr:   [bool; 256],
    pub survive_arr: [bool; 256],
}
impl Rules {
    pub fn new(birth: &[u8], survive: &[u8]) -> Self {
        let birth_arr: [bool; 256] = core::array::from_fn(|x| {
            let alive_neighbors = x.count_ones();

            #[allow(clippy::cast_possible_truncation)]
            let alive_neighbors = alive_neighbors as u8;

            birth.contains(&alive_neighbors)
        });

        let survive_arr: [bool; 256] = core::array::from_fn(|x| {
            let alive_neighbors = x.count_ones();

            #[allow(clippy::cast_possible_truncation)]
            let alive_neighbors = alive_neighbors as u8;

            survive.contains(&alive_neighbors)
        });

        Self { birth: birth.into(),
               survive: survive.into(),

               birth_arr,
               survive_arr }
    }

    pub fn from_string(input: &str) -> Result<Self, &'static str> {
        static RULES_REGEX: &Lazy<Regex> = regex!(r"^B([0-8]+)/S([0-8]+)$");

        let (birth, survive) = if let Some(captures) = RULES_REGEX.captures(input) {
            #[allow(clippy::cast_possible_truncation)]
            (captures[1].chars()
                        .map(|c| c.to_digit(10).map(|x| x as u8).unwrap())
                        .collect::<Vec<_>>(),
             #[allow(clippy::cast_possible_truncation)]
             captures[2].chars()
                        .map(|c| c.to_digit(10).map(|x| x as u8).unwrap())
                        .collect::<Vec<_>>())
        } else {
            return Err("Invalid rules")
        };

        Ok(Self::new(&birth, &survive))
    }
}
impl Default for Rules {
    fn default() -> Self {
        Self::new(&[3], &[2, 3])
    }
}
