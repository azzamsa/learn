pub fn to_typed(input: &str) -> String {
    input.split_whitespace().map(|word| {
        let name = word
            .replace("-", "_")
            .replace(":", "__");
        format!("C.{}", name)
    }).collect::<Vec<_>>().join(", ")
    // @TODO: replace with  `}).intersperse(", ").collect()` once `Iterator::intersperse` is stable
}

pub fn to_css(input: &str) -> String {
    input
        .replace(",", "")
        .replace("__", ":")
        .replace("_", "-")
        .replace("C.", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    // to_typed
    #[test]
    fn test_transform() {
        let input_string = "text-white font-bold py-2";
        let expected_result = "C.text_white, C.font_bold, C.py_2".to_string();
        assert_eq!(to_typed(input_string), expected_result);
    }

    #[test]
    fn test_transform_double_dash() {
        let input_string = "bg-gray-100 text-white font-bold py-2";
        let expected_result =
            "C.bg_gray_100, C.text_white, C.font_bold, C.py_2".to_string();
        assert_eq!(to_typed(input_string), expected_result);
    }

    #[test]
    fn test_transform_one_word() {
        let input_string = "flex";
        let expected_result = "C.flex".to_string();
        assert_eq!(to_typed(input_string), expected_result);
    }

    #[test]
    fn test_transform_two_word() {
        let input_string = "flex flex-col";
        let expected_result = "C.flex, C.flex_col".to_string();
        assert_eq!(to_typed(input_string), expected_result);
    }

    #[test]
    fn test_transform_with_semicolon() {
        let input_string = "text-white font-bold py-2 hover:bg-blue-light rounded";
        let expected_result =
            "C.text_white, C.font_bold, C.py_2, C.hover__bg_blue_light, C.rounded"
                .to_string();
        assert_eq!(to_typed(input_string), expected_result);
    }

    // to_css
    #[test]
    fn test_transform2() {
        let input_string = "C.text_white, C.font_bold, C.py_2";
        let expected_result = "text-white font-bold py-2".to_string();
        assert_eq!(to_css(input_string), expected_result);
    }

    #[test]
    fn test_transform2_one_word() {
        let input_string = "C.flex";
        let expected_result = "flex".to_string();
        assert_eq!(to_css(input_string), expected_result);
    }

    #[test]
    fn test_transform2_two_word() {
        let input_string = "C.flex, C.flex_col";
        let expected_result = "flex flex-col".to_string();
        assert_eq!(to_css(input_string), expected_result);
    }

    #[test]
    fn test_transform2_double_undescore_separate_place() {
        let input_string = "C.bg_gray_100, C.text_white, C.font_bold, C.py_2";
        let expected_result = "bg-gray-100 text-white font-bold py-2".to_string();
        assert_eq!(to_css(input_string), expected_result);
    }

    #[test]
    fn test_transform2_with_double_underscore_one_place() {
        let input_string =
            "C.text_white, C.font_bold, C.py_2, C.hover__bg_blue_light, C.rounded";
        let expected_result =
            "text-white font-bold py-2 hover:bg-blue-light rounded".to_string();
        assert_eq!(to_css(input_string), expected_result);
    }

    #[test]
    fn test_transform2_with_double_underscore_one_place1() {
        let input_string =
            "C.focus__text-white, C.font_bold, C.py_2, C.hover__bg_blue_light C.rounded";
        let expected_result =
            "focus:text-white font-bold py-2 hover:bg-blue-light rounded".to_string();
        assert_eq!(to_css(input_string), expected_result);
    }
}
