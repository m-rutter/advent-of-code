//! Advent of Code (AoC) 2017 solutions library written in Rust.
//! Personal learning project.
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

/// AoC config
#[derive(Debug, Default)]
pub struct Config {
    day: u8,
    input: ConfigInput,
}

#[derive(Debug)]
enum ConfigInput {
    Stdin,
    Sample,
    Input(String),
    InputPath(String),
}

impl Default for ConfigInput {
    fn default() -> ConfigInput {
        ConfigInput::Stdin
    }
}

/// Solution for a day in AoC 2017
pub struct AoCSolution {
    pub part_one: String,
    pub part_two: String,
}

impl Config {
    /// Creates a instance of AoC Config defaulting to using stdin
    /// # Example
    /// ```
    /// use aoc_2017::Config;
    ///
    /// let config = Config::new(1);
    /// ```
    pub fn new(day: u8) -> Config {
        Config {
            day: day,
            ..Config::default()
        }
    }

    /// Set the day to compute
    /// # Example
    /// ```
    /// use aoc_2017::Config;
    ///
    /// let mut config = Config::new(1);
    ///
    /// config.with_day(23);
    /// ```
    pub fn with_day(&mut self, day: u8) {
        self.day = day;
    }

    /// Use stdin for providing input
    /// # Example
    /// ```
    /// use aoc_2017::Config;
    ///
    /// let mut config = Config::new(1);
    ///
    /// config.with_stdin();
    /// ```
    pub fn with_stdin(&mut self) {
        self.input = ConfigInput::Stdin;
    }

    /// Provide the input to evaluate
    /// # Example
    /// ```
    /// use aoc_2017::Config;
    ///
    /// let mut config = Config::new(1);
    ///
    /// config.with_input("42347238904723984524532543234");
    /// ```
    pub fn with_input(&mut self, input: &str) {
        self.input = ConfigInput::Input(String::from(input));
    }

    /// Provide file path to read input
    /// # Example
    /// ```
    /// use aoc_2017::Config;
    ///
    /// let mut config = Config::new(1);
    ///
    /// config.with_input_file_path("./input");
    /// ```
    pub fn with_input_file_path(&mut self, input: &str) {
        self.input = ConfigInput::InputPath(String::from(input));
    }

    /// Use the built-in sample input
    /// # Example
    /// ```
    /// use aoc_2017::Config;
    ///
    /// let mut config = Config::new(1);
    ///
    /// config.with_sample_input();
    /// ```
    pub fn with_sample_input(&mut self) {
        self.input = ConfigInput::Sample;
    }
}

/// Computes the solution to a day in Advent of Code 2017
/// # Examples
/// ```rust
/// use aoc_2017::{solve_day, Config};
///
/// let mut config = Config::new(1);
/// config.with_input("649713959682898259577777982349515784822684939966191359164369933435366431847754488661965363557985166219358714739318371382388296151195361571216131925158492441461844687324923315381358331571577613789649166486152237945917987977793891739865149734755993241361886336926538482271124755359572791451335842534893192693558659991171983849285489139421425933638614884415896938914992732492192458636484523228244532331587584779552788544667253577324649915274115924611758345676183443982992733966373498385685965768929241477983727921279826727976872556315428434799161759734932659829934562339385328119656823483954856427365892627728163524721467938449943358192632262354854593635831559352247443975945144163183563723562891357859367964126289445982135523535923113589316417623483631637569291941782992213889513714525342468563349385271884221685549996534333765731243895662624829924982971685443825366827923589435254514211489649482374876434549682785459698885521673258939413255158196525696236457911447599947449665542554251486847388823576937167237476556782133227279324526834946534444718161524129285919477959937684728882592779941734186144138883994322742484853925383518651687147246943421311287324867663698432546619583638976637733345251834869985746385371617743498627111441933546356934671639545342515392536574744795732243617113574641284231928489312683617154536648219244996491745718658151648246791826466973654765284263928884137863647623237345882469142933142637583644258427416972595241737254449718531724176538648369253796688931245191382956961544775856872281317743828552629843551844927913147518377362266554334386721313244223233396453291224932499277961525785755863852487141946626663835195286762947172384186667439516367219391823774338692151926472717373235612911848773387771244144969149482477519437822863422662157461968444281972353149695515494992537927492111388193837553844671719291482442337761321272333982924289323437277224565149928416255435841327756139118119744528993269157174414264387573331116323982614862952264597611885999285995516357519648695594299657387614793341626318866519144574571816535351149394735916975448425618171572917195165594323552199346814729617189679698944337146");
///  
/// match solve_day(config) {
///     Ok(solution) => {
///         println!("Part 1 solution for day 1: {}", solution.part_one);
///         println!("Part 2 solution for day 1: {}", solution.part_two);
///     },
///     Err(error) => {
///         eprintln!("{}", error)
///     }
/// };
///
/// ```
pub fn solve_day(config: Config) -> Result<AoCSolution, String> {
    match config.day {
        1 => day01::run(config),
        2 => day02::run(config),
        3 => day03::run(config),
        4 => day04::run(config),
        5 => day05::run(config),
        6 => day06::run(config),
        7 => day07::run(config),
        _ => return Err(format!("Day {} is not yet supported", config.day)),
    }

    Ok(AoCSolution {
        part_one: String::new(),
        part_two: String::new(),
    })
}
