//! EditCommand の実行・復元ロジック（Phase 6.5）
//!
//! EditCommand に apply() と revert() メソッドを提供し、
//! Undo/Redo システムをサポートします。

use crate::editor::{EditCommand, SpriteDocument};

/// EditCommand 実行エラー
#[derive(Clone, Debug)]
pub struct CommandExecutionError {
    pub message: String,
}

impl std::fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Command execution error: {}", self.message)
    }
}

impl std::error::Error for CommandExecutionError {}

impl EditCommand {
    /// コマンドを実行（ドキュメント状態を変更）
    pub fn apply(&self, doc: &mut SpriteDocument) -> Result<(), CommandExecutionError> {
        match self {
            // ピクセル描画を実行
            EditCommand::PaintStroke { layer_id, pixels } => {
                // アクティブレイヤーを取得
                if let Some(layer) = doc
                    .layers
                    .layers_mut()
                    .iter_mut()
                    .find(|l| l.id == *layer_id)
                {
                    // すべてのピクセルを適用
                    for (x, y, color) in pixels {
                        layer.sprite.set_pixel(*x, *y, color).map_err(|e| {
                            CommandExecutionError {
                                message: format!("Failed to paint pixel: {}", e),
                            }
                        })?;
                    }
                    doc.invalidate_composite();
                    Ok(())
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            // レイヤー追加は既に実装済み（LayerStack::add_layer）
            EditCommand::AddLayer { .. } => Err(CommandExecutionError {
                message: "AddLayer must be applied via LayerStack API".to_string(),
            }),

            // レイヤー削除
            EditCommand::DeleteLayer { layer_id, .. } => {
                if let Some(idx) = doc.layers.layers().iter().position(|l| l.id == *layer_id) {
                    // アクティブレイヤーをここで削除することにする
                    // 実装注：実際には LayerStack::delete_layer_by_id を新規実装すべき
                    Err(CommandExecutionError {
                        message: "DeleteLayer must be applied via LayerStack API".to_string(),
                    })
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            // レイヤー不透明度変更
            EditCommand::SetLayerOpacity {
                layer_id,
                new_opacity,
                ..
            } => {
                if let Some(layer) = doc
                    .layers
                    .layers_mut()
                    .iter_mut()
                    .find(|l| l.id == *layer_id)
                {
                    layer.set_opacity(*new_opacity);
                    doc.invalidate_composite();
                    Ok(())
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            // レイヤーブレンドモード変更
            EditCommand::SetLayerBlendMode {
                layer_id, new_mode, ..
            } => {
                if let Some(layer) = doc
                    .layers
                    .layers_mut()
                    .iter_mut()
                    .find(|l| l.id == *layer_id)
                {
                    layer.blend_mode = *new_mode;
                    doc.invalidate_composite();
                    Ok(())
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            // レイヤー可視性変更
            EditCommand::SetLayerVisibility {
                layer_id,
                new_visible,
                ..
            } => {
                if let Some(layer) = doc
                    .layers
                    .layers_mut()
                    .iter_mut()
                    .find(|l| l.id == *layer_id)
                {
                    layer.visible = *new_visible;
                    doc.invalidate_composite();
                    Ok(())
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            // レイヤーロック状態変更
            EditCommand::SetLayerLocked {
                layer_id,
                new_locked,
                ..
            } => {
                if let Some(layer) = doc
                    .layers
                    .layers_mut()
                    .iter_mut()
                    .find(|l| l.id == *layer_id)
                {
                    layer.locked = *new_locked;
                    Ok(())
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            // フレーム操作（未実装）
            EditCommand::AddFrame { .. } => Err(CommandExecutionError {
                message: "AddFrame not yet implemented".to_string(),
            }),
            EditCommand::DeleteFrame { .. } => Err(CommandExecutionError {
                message: "DeleteFrame not yet implemented".to_string(),
            }),
            EditCommand::SetFrameDuration { .. } => Err(CommandExecutionError {
                message: "SetFrameDuration not yet implemented".to_string(),
            }),

            // その他
            EditCommand::MoveLayer { .. } => Err(CommandExecutionError {
                message: "MoveLayer must be applied via LayerStack API".to_string(),
            }),
        }
    }

    /// コマンドを復元（Undo 時に呼び出す）
    pub fn revert(&self, doc: &mut SpriteDocument) -> Result<(), CommandExecutionError> {
        match self {
            // ピクセル描画を復元
            EditCommand::PaintStroke { layer_id, pixels } => {
                // NOTE: PaintStroke には old_color がないため、完全な復元は不可能
                // 改善案：EditCommand::PaintStroke に old_colors フィールドを追加
                Err(CommandExecutionError {
                    message: "PaintStroke requires old_colors for proper revert".to_string(),
                })
            }

            // 以下は apply の逆操作
            EditCommand::SetLayerOpacity {
                layer_id,
                old_opacity,
                ..
            } => {
                if let Some(layer) = doc
                    .layers
                    .layers_mut()
                    .iter_mut()
                    .find(|l| l.id == *layer_id)
                {
                    layer.set_opacity(*old_opacity);
                    doc.invalidate_composite();
                    Ok(())
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            EditCommand::SetLayerBlendMode {
                layer_id, old_mode, ..
            } => {
                if let Some(layer) = doc
                    .layers
                    .layers_mut()
                    .iter_mut()
                    .find(|l| l.id == *layer_id)
                {
                    layer.blend_mode = *old_mode;
                    doc.invalidate_composite();
                    Ok(())
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            EditCommand::SetLayerVisibility {
                layer_id,
                old_visible,
                ..
            } => {
                if let Some(layer) = doc
                    .layers
                    .layers_mut()
                    .iter_mut()
                    .find(|l| l.id == *layer_id)
                {
                    layer.visible = *old_visible;
                    doc.invalidate_composite();
                    Ok(())
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            EditCommand::SetLayerLocked {
                layer_id,
                old_locked,
                ..
            } => {
                if let Some(layer) = doc
                    .layers
                    .layers_mut()
                    .iter_mut()
                    .find(|l| l.id == *layer_id)
                {
                    layer.locked = *old_locked;
                    Ok(())
                } else {
                    Err(CommandExecutionError {
                        message: format!("Layer {} not found", layer_id),
                    })
                }
            }

            // その他はまだ未実装
            _ => Err(CommandExecutionError {
                message: "This command type does not support revert yet".to_string(),
            }),
        }
    }
}

/// コマンド実行の検証
pub fn validate_command(cmd: &EditCommand) -> Result<(), CommandExecutionError> {
    match cmd {
        EditCommand::PaintStroke { pixels, .. } => {
            if pixels.is_empty() {
                Err(CommandExecutionError {
                    message: "PaintStroke must contain at least one pixel".to_string(),
                })
            } else {
                Ok(())
            }
        }

        EditCommand::SetLayerOpacity {
            new_opacity,
            old_opacity,
            ..
        } => {
            if *new_opacity < 0.0 || *new_opacity > 1.0 {
                Err(CommandExecutionError {
                    message: "Opacity must be between 0.0 and 1.0".to_string(),
                })
            } else if *old_opacity < 0.0 || *old_opacity > 1.0 {
                Err(CommandExecutionError {
                    message: "Old opacity must be between 0.0 and 1.0".to_string(),
                })
            } else {
                Ok(())
            }
        }

        _ => Ok(()), // その他は無条件で OK
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ColorMode;

    #[test]
    fn test_set_opacity_apply() {
        let sprite = crate::resource::SpriteData::new(32, 32, ColorMode::FullColor);
        let mut doc = crate::editor::SpriteDocument::new(0, "Test", sprite);

        let cmd = EditCommand::SetLayerOpacity {
            layer_id: 0,
            old_opacity: 1.0,
            new_opacity: 0.5,
        };

        assert!(cmd.apply(&mut doc).is_ok());
        assert_eq!(doc.layers.active_layer().unwrap().opacity, 0.5);
    }

    #[test]
    fn test_set_opacity_revert() {
        let sprite = crate::resource::SpriteData::new(32, 32, ColorMode::FullColor);
        let mut doc = crate::editor::SpriteDocument::new(0, "Test", sprite);

        let cmd = EditCommand::SetLayerOpacity {
            layer_id: 0,
            old_opacity: 1.0,
            new_opacity: 0.5,
        };

        cmd.apply(&mut doc).unwrap();
        cmd.revert(&mut doc).unwrap();
        assert_eq!(doc.layers.active_layer().unwrap().opacity, 1.0);
    }

    #[test]
    fn test_set_visibility_apply() {
        let sprite = crate::resource::SpriteData::new(32, 32, ColorMode::FullColor);
        let mut doc = crate::editor::SpriteDocument::new(0, "Test", sprite);

        let cmd = EditCommand::SetLayerVisibility {
            layer_id: 0,
            old_visible: true,
            new_visible: false,
        };

        assert!(cmd.apply(&mut doc).is_ok());
        assert!(!doc.layers.active_layer().unwrap().visible);
    }

    #[test]
    fn test_validate_empty_paint_stroke() {
        let cmd = EditCommand::PaintStroke {
            layer_id: 0,
            pixels: vec![],
        };

        assert!(validate_command(&cmd).is_err());
    }

    #[test]
    fn test_validate_invalid_opacity() {
        let cmd = EditCommand::SetLayerOpacity {
            layer_id: 0,
            old_opacity: 1.0,
            new_opacity: 1.5, // > 1.0
        };

        assert!(validate_command(&cmd).is_err());
    }
}
