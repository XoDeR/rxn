use rx_nodes::{Camera2D, Node2D, SceneNode, Sprite2D};
use rx_runtime::RuntimeInner;
use rx_structs::Vector2;

fn main() {
    let mut runtime = RuntimeInner {
        nodes: Default::default(),
    };

    // Root node
    let mut root = SceneNode::new(Node2D::new().into());
    root.set_name("Root");
    let root_id = runtime.nodes.insert(root);

    // Background sprite child, drawn below "unit"
    let mut bg_data = Sprite2D::new();
    bg_data.z_index = 0;
    let mut bg = SceneNode::new(bg_data.into());
    bg.set_name("bg");
    bg.parent = root_id;
    let bg_id = runtime.nodes.insert(bg);

    // Unit sprite child, drawn above "bg"
    let mut unit_data = Sprite2D::new();
    unit_data.position = Vector2::new(10.0, 20.0);
    unit_data.z_index = 1;
    let mut unit = SceneNode::new(unit_data.into());
    unit.set_name("unit");
    unit.parent = root_id;
    let unit_id = runtime.nodes.insert(unit);

    // Camera child
    let mut camera_data = Camera2D::new();
    camera_data.zoom = 1.0;
    camera_data.active = true;
    let mut camera = SceneNode::new(camera_data.into());
    camera.set_name("camera");
    camera.parent = root_id;
    let camera_id = runtime.nodes.insert(camera);

    // Link children to the root
    let root = runtime.nodes.get_mut(root_id).unwrap();
    root.add_child(bg_id);
    root.add_child(unit_id);
    root.add_child(camera_id);

    // Mutate the unit sprite through its typed accessor
    runtime
        .nodes
        .get_mut(unit_id)
        .unwrap()
        .with_typed_mut::<Sprite2D, _>(|s| s.flip_x = true);

    println!("{} nodes in the arena:", runtime.nodes.len());
    for (id, node) in runtime.nodes.iter() {
        println!(
            "  {:?} {:<8} type={:?} children={}",
            id,
            node.get_name(),
            node.node_type(),
            node.get_children_ids().len()
        );
    }
}
