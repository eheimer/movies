use movies::components::*;
use movies::components::episode::Episode;
use movies::theme::Theme;

#[test]
fn test_component_trait_signature() {
    // Test that all components implement the new trait signature with width, height, theme, is_selected
    let theme = Theme::default();
    
    // Episode component (single-line)
    let episode = Episode::new("Test Episode".to_string(), false, true, false);
    let result = episode.render(50, 1, &theme, false);
    assert_eq!(result.len(), 1, "Episode should render exactly one row");
    
    // Category component (single-line)  
    let category = Category::new("Test Category".to_string(), 1, 1, CategoryType::Series);
    let result = category.render(50, 1, &theme, false);
    assert_eq!(result.len(), 1, "Category should render exactly one row");
    
    // Browser component (multi-line)
    let browser = Browser::new(
        (0, 0),
        50,
        vec![category],
        vec![episode],
    );
    let result = browser.render(50, 3, &theme, false);
    assert!(result.len() <= 3, "Browser should respect height constraint");
    
    // Scrollbar component
    let scrollbar = Scrollbar::new(10, 5, 0);
    let result = scrollbar.render(1, 5, &theme, false);
    assert_eq!(result.len(), 5, "Scrollbar should render to specified height");
}

#[test]
fn test_single_line_components_ignore_height() {
    let theme = Theme::default();
    
    // Episode should render identically regardless of height
    let episode = Episode::new("Test".to_string(), false, true, false);
    let result1 = episode.render(30, 1, &theme, false);
    let result2 = episode.render(30, 10, &theme, false);
    assert_eq!(result1, result2, "Episode output should be identical regardless of height");
    assert_eq!(result1.len(), 1, "Episode should always render exactly one row");
    
    // Category should render identically regardless of height
    let category = Category::new("Test".to_string(), 1, 1, CategoryType::Series);
    let result1 = category.render(30, 1, &theme, false);
    let result2 = category.render(30, 10, &theme, false);
    assert_eq!(result1, result2, "Category output should be identical regardless of height");
    assert_eq!(result1.len(), 1, "Category should always render exactly one row");
}

#[test]
fn test_browser_uses_height_parameter() {
    let theme = Theme::default();
    
    // Create browser with more content than can fit in small height
    let categories = vec![
        Category::new("Series 1".to_string(), 1, 1, CategoryType::Series),
        Category::new("Series 2".to_string(), 2, 1, CategoryType::Series),
    ];
    let episodes = vec![
        Episode::new("Episode 1".to_string(), false, true, false),
        Episode::new("Episode 2".to_string(), false, true, false),
        Episode::new("Episode 3".to_string(), false, true, false),
    ];
    
    let browser = Browser::new((0, 0), 50, categories, episodes);
    
    // Test with small height - should limit output
    let result_small = browser.render(50, 2, &theme, false);
    assert_eq!(result_small.len(), 2, "Browser should respect small height constraint");
    
    // Test with large height - should show all content
    let result_large = browser.render(50, 10, &theme, false);
    assert!(result_large.len() <= 10, "Browser should respect large height constraint");
    assert!(result_large.len() >= 2, "Browser should show available content");
}

#[test]
fn test_scrollbar_uses_height_parameter() {
    let theme = Theme::default();
    
    let scrollbar = Scrollbar::new(20, 5, 0);
    
    // Test different heights
    let result1 = scrollbar.render(1, 3, &theme, false);
    assert_eq!(result1.len(), 3, "Scrollbar should render to height 3");
    
    let result2 = scrollbar.render(1, 8, &theme, false);
    assert_eq!(result2.len(), 8, "Scrollbar should render to height 8");
}

#[test]
fn test_browser_dimension_constraints() {
    let theme = Theme::default();
    
    let browser = Browser::new(
        (0, 0),
        25,  // Use same width as render call
        vec![Category::new("Test".to_string(), 1, 1, CategoryType::Series)],
        vec![Episode::new("Test".to_string(), false, true, false)],
    );
    
    // Test that output respects height constraints
    let result = browser.render(25, 3, &theme, false);
    assert!(result.len() <= 3, "Browser output should not exceed height");
    
    // Browser uses its internal width, so output should match that
    for row in &result {
        assert!(row.len() <= 25, "Browser row should not exceed its configured width");
    }
}

#[test]
fn test_browser_methods_require_height() {
    let mut browser = Browser::new(
        (0, 0),
        50,
        vec![Category::new("Test".to_string(), 1, 1, CategoryType::Series)],
        vec![
            Episode::new("Episode 1".to_string(), false, true, false),
            Episode::new("Episode 2".to_string(), false, true, false),
            Episode::new("Episode 3".to_string(), false, true, false),
        ],
    );
    
    // Test that methods that need height parameter work correctly
    assert!(!browser.needs_scrollbar(10), "Should not need scrollbar with large height");
    assert!(browser.needs_scrollbar(2), "Should need scrollbar with small height");
    
    assert_eq!(browser.content_width(10), 50, "Content width should equal full width without scrollbar");
    assert_eq!(browser.content_width(2), 49, "Content width should be reduced with scrollbar");
    
    assert_eq!(browser.visible_items(10), 4, "Should show all 4 items with large height");
    assert_eq!(browser.visible_items(2), 2, "Should show 2 items with small height");
    
    browser.clamp_first_visible_item(3);
    browser.ensure_selection_visible(3);
    // These methods should not panic and should work with height parameter
}

#[test]
fn test_empty_browser_edge_cases() {
    let theme = Theme::default();
    
    // Test empty browser
    let empty_browser = Browser::new((0, 0), 30, vec![], vec![]);
    let result = empty_browser.render(30, 5, &theme, false);
    assert_eq!(result.len(), 5, "Empty browser should return empty rows for full height");
    
    // Test zero dimensions
    let result_zero = empty_browser.render(0, 0, &theme, false);
    assert_eq!(result_zero.len(), 0, "Zero height should return empty result");
}