#[cfg(all(feature = "stdio_game_runner", feature = "ratatui_game_runner"))]
compile_error!("feature \"stdio_game_runner\" and feature \"ratatui_game_runner\" cannot be enabled at the same time");

#[cfg(not(any(feature = "stdio_game_runner", feature = "ratatui_game_runner")))]
compile_error!("feature \"stdio_game_runner\" or feature \"ratatui_game_runner\" must be enabled");
