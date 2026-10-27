use super::*;

#[derive(Debug, Clone)]
pub enum ResourceCommand {
    CreateTexture {
        request: RenderRequestID,
        id: TextureID,
        source: String,
        reserved: bool,
    },
    SetSceneResourceRefs {
        textures: Vec<(TextureID, Vec<NodeID>)>,
    },
    SetTextureReserved {
        id: TextureID,
        reserved: bool,
    },
    DropTexture {
        id: TextureID,
    },
}

#[derive(Debug, Clone)]
pub enum Command2D {
    UpsertSprite {
        node: NodeID,
        sprite: Sprite2DCommand,
    },
    UpsertRect {
        node: NodeID,
        rect: Rect2DCommand,
    },
    RemoveNode {
        node: NodeID,
    },
    SetCamera {
        camera: Camera2DState,
    },
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum RenderCommand {
    Resource(ResourceCommand),
    TwoD(Command2D),
}

#[derive(Debug, Clone)]
pub enum RenderEvent {
    TextureCreated {
        request: RenderRequestID,
        id: TextureID,
    },
    TextureLoaded {
        id: TextureID,
    },
    TextureDropped {
        id: TextureID,
    },
    Failed {
        request: RenderRequestID,
        reason: String,
    },
}

pub trait RenderBridge {
    fn submit(&mut self, command: RenderCommand);

    fn submit_many<I>(&mut self, commands: I)
    where
        I: IntoIterator<Item = RenderCommand>,
    {
        for command in commands {
            self.submit(command);
        }
    }

    fn drain_events(&mut self, out: &mut Vec<RenderEvent>);
}
