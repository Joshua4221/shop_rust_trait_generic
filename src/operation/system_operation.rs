pub trait SystemHandler<T, U> {
    type MenuOption;

    fn show_meun();

    fn parse_input(input: i32) -> Option<Self::MenuOption>;

    fn handle_option(
        option: Self::MenuOption,
        store: &mut T,
        buyer: &mut U,
    ) -> Result<bool, String>;
}
