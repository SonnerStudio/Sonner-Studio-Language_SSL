//! SSL 4.0 Mobile UI Module
//!
//! Cross-platform UI components for mobile apps.

use serde::{Serialize, Deserialize};

/// UI Component tree node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIComponent {
    /// Vertical stack layout
    VStack {
        children: Vec<UIComponent>,
        spacing: f32,
        alignment: Alignment,
    },
    /// Horizontal stack layout
    HStack {
        children: Vec<UIComponent>,
        spacing: f32,
        alignment: Alignment,
    },
    /// Text label
    Text {
        content: String,
        style: TextStyle,
    },
    /// Button
    Button {
        label: String,
        action: String, // SSL function name
        style: ButtonStyle,
    },
    /// Text input field
    TextField {
        placeholder: String,
        binding: String, // Variable name
        style: TextFieldStyle,
    },
    /// Image
    Image {
        source: ImageSource,
        fit: ImageFit,
    },
    /// Spacer (flexible space)
    Spacer {
        min_size: Option<f32>,
    },
    /// Divider line
    Divider {
        color: Option<String>,
    },
    /// Scrollable container
    ScrollView {
        content: Box<UIComponent>,
        direction: ScrollDirection,
    },
    /// List of items
    List {
        items: String, // SSL variable name
        item_view: Box<UIComponent>,
    },
    /// Navigation bar
    NavigationBar {
        title: String,
        leading: Option<Box<UIComponent>>,
        trailing: Option<Box<UIComponent>>,
    },
    /// Custom component
    Custom {
        name: String,
        props: Vec<(String, String)>,
    },
}

/// Alignment options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Alignment {
    Leading,
    Center,
    Trailing,
    Top,
    Bottom,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Center
    }
}

/// Text styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub color: Option<String>,
    pub line_limit: Option<u32>,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            font_weight: FontWeight::Regular,
            color: None,
            line_limit: None,
        }
    }
}

/// Font weight options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FontWeight {
    Thin,
    Light,
    Regular,
    Medium,
    Semibold,
    Bold,
    Heavy,
    Black,
}

/// Button styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonStyle {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub full_width: bool,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Filled,
            size: ButtonSize::Medium,
            full_width: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ButtonVariant {
    Filled,
    Outlined,
    Text,
    Tonal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// Text field styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFieldStyle {
    pub variant: TextFieldVariant,
    pub keyboard_type: KeyboardType,
    pub secure: bool,
}

impl Default for TextFieldStyle {
    fn default() -> Self {
        Self {
            variant: TextFieldVariant::Outlined,
            keyboard_type: KeyboardType::Default,
            secure: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TextFieldVariant {
    Outlined,
    Filled,
    Standard,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KeyboardType {
    Default,
    Email,
    Number,
    Phone,
    Url,
    Password,
}

/// Image source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageSource {
    Asset(String),
    Url(String),
    SystemIcon(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ImageFit {
    Fill,
    Fit,
    Cover,
    None,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

/// Generate SwiftUI code from UI component
pub fn to_swiftui(component: &UIComponent) -> String {
    match component {
        UIComponent::VStack { children, spacing, alignment } => {
            let children_code: Vec<String> = children.iter()
                .map(|c| to_swiftui(c))
                .collect();
            format!(
                "VStack(alignment: .{}, spacing: {}) {{\n{}\n}}",
                alignment_to_swift(alignment),
                spacing,
                children_code.join("\n")
            )
        }
        UIComponent::HStack { children, spacing, alignment } => {
            let children_code: Vec<String> = children.iter()
                .map(|c| to_swiftui(c))
                .collect();
            format!(
                "HStack(alignment: .{}, spacing: {}) {{\n{}\n}}",
                alignment_to_swift(alignment),
                spacing,
                children_code.join("\n")
            )
        }
        UIComponent::Text { content, style } => {
            format!(
                "Text(\"{}\")\n    .font(.system(size: {}))\n    .fontWeight(.{})",
                content,
                style.font_size,
                font_weight_to_swift(&style.font_weight)
            )
        }
        UIComponent::Button { label, action, style: _ } => {
            format!(
                "Button(\"{}\") {{\n    SSLBridge.shared.{}()\n}}",
                label, action
            )
        }
        UIComponent::Spacer { min_size } => {
            match min_size {
                Some(size) => format!("Spacer(minLength: {})", size),
                None => "Spacer()".to_string(),
            }
        }
        _ => "// TODO: Component not yet supported".to_string(),
    }
}

fn alignment_to_swift(alignment: &Alignment) -> &'static str {
    match alignment {
        Alignment::Leading => "leading",
        Alignment::Center => "center",
        Alignment::Trailing => "trailing",
        Alignment::Top => "top",
        Alignment::Bottom => "bottom",
    }
}

fn font_weight_to_swift(weight: &FontWeight) -> &'static str {
    match weight {
        FontWeight::Thin => "thin",
        FontWeight::Light => "light",
        FontWeight::Regular => "regular",
        FontWeight::Medium => "medium",
        FontWeight::Semibold => "semibold",
        FontWeight::Bold => "bold",
        FontWeight::Heavy => "heavy",
        FontWeight::Black => "black",
    }
}

/// Generate Jetpack Compose code from UI component
pub fn to_compose(component: &UIComponent) -> String {
    match component {
        UIComponent::VStack { children, spacing, alignment } => {
            let children_code: Vec<String> = children.iter()
                .map(|c| to_compose(c))
                .collect();
            format!(
                "Column(\n    horizontalAlignment = Alignment.{},\n    verticalArrangement = Arrangement.spacedBy({}.dp)\n) {{\n{}\n}}",
                alignment_to_compose(alignment),
                spacing,
                children_code.join("\n")
            )
        }
        UIComponent::HStack { children, spacing, alignment } => {
            let children_code: Vec<String> = children.iter()
                .map(|c| to_compose(c))
                .collect();
            format!(
                "Row(\n    verticalAlignment = Alignment.{},\n    horizontalArrangement = Arrangement.spacedBy({}.dp)\n) {{\n{}\n}}",
                alignment_to_compose(alignment),
                spacing,
                children_code.join("\n")
            )
        }
        UIComponent::Text { content, style } => {
            format!(
                "Text(\n    text = \"{}\",\n    fontSize = {}.sp,\n    fontWeight = FontWeight.{}\n)",
                content,
                style.font_size,
                font_weight_to_compose(&style.font_weight)
            )
        }
        UIComponent::Button { label, action, style: _ } => {
            format!(
                "Button(onClick = {{ SSLBridge.{}() }}) {{\n    Text(\"{}\")\n}}",
                action, label
            )
        }
        UIComponent::Spacer { min_size } => {
            match min_size {
                Some(size) => format!("Spacer(modifier = Modifier.height({}.dp))", size),
                None => "Spacer(modifier = Modifier.weight(1f))".to_string(),
            }
        }
        _ => "// TODO: Component not yet supported".to_string(),
    }
}

fn alignment_to_compose(alignment: &Alignment) -> &'static str {
    match alignment {
        Alignment::Leading => "Start",
        Alignment::Center => "CenterHorizontally",
        Alignment::Trailing => "End",
        Alignment::Top => "Top",
        Alignment::Bottom => "Bottom",
    }
}

fn font_weight_to_compose(weight: &FontWeight) -> &'static str {
    match weight {
        FontWeight::Thin => "Thin",
        FontWeight::Light => "Light",
        FontWeight::Regular => "Normal",
        FontWeight::Medium => "Medium",
        FontWeight::Semibold => "SemiBold",
        FontWeight::Bold => "Bold",
        FontWeight::Heavy => "ExtraBold",
        FontWeight::Black => "Black",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_component() {
        let text = UIComponent::Text {
            content: "Hello".to_string(),
            style: TextStyle::default(),
        };
        
        let swift = to_swiftui(&text);
        assert!(swift.contains("Text(\"Hello\")"));
        
        let compose = to_compose(&text);
        assert!(compose.contains("text = \"Hello\""));
    }
    
    #[test]
    fn test_vstack() {
        let vstack = UIComponent::VStack {
            children: vec![
                UIComponent::Text {
                    content: "Title".to_string(),
                    style: TextStyle::default(),
                },
            ],
            spacing: 16.0,
            alignment: Alignment::Center,
        };
        
        let swift = to_swiftui(&vstack);
        assert!(swift.contains("VStack"));
    }
}
