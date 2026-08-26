pub fn default_tools() -> ToolRegistry {
    let mut registry = ToolRegistry::new();

    registry.register(CalculatorTool);

    registry
}
