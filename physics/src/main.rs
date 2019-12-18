struct Coordinates{
    x: f32,
    y: f32,
    z: f32
}

struct User {
    position: Coordinates,
    speed: f32,
    mass: f32,
    can_move: bool
}


// On fait une énum pour répertorier tous les types de Mesh qu'on peut créer avec ncollide
enum MeshType {
    Ball,
    Capsule,
    Compound,
    ConvexHull,
    Cuboid,
    HeightField,
    Plane,
    Polyline,
    Segment,
    TriMesh,
    Triangle,
}

// Fait des trucs inutiles pour le moment
fn process_mesh(event: MeshType) {
    match event {
        MeshType::Ball => println!("Ball"),
        MeshType::Capsule => println!("Capsule"),
        MeshType::Compound => println!("Compound"),
        MeshType::ConvexHull => println!("ConvexHull"),
        MeshType::Cuboid => println!("Cuboid"),
        MeshType::HeightField => println!("HeightField"),
        MeshType::Plane => println!("Plane"),
        MeshType::Polyline => println!("Polyline"),
        MeshType::Segment => println!("Segment"),
        MeshType::TriMesh => println!("TriMesh"),
        MeshType::Triangle => println!("Triangle"),
    }
}

fn main() {
    let itsaball  = MeshType::Ball;
    let itsacapsule  = MeshType::Capsule;
    let itsacompound  = MeshType::Compound;
    let itsaconvexhull  = MeshType::ConvexHull;
    let itsacuboid  = MeshType::Cuboid;
    let itsaheightfield  = MeshType::HeightField;
    let itsaplane  = MeshType::Plane;
    let itsapolyline  = MeshType::Polyline;
    let itsasegment  = MeshType::Segment;
    let itsatrimesh  = MeshType::TriMesh;
    let itsatriangle  = MeshType::Triangle;

    process_mesh(itsaball);
    process_mesh(itsacapsule);
    process_mesh(itsacompound);
    process_mesh(itsaconvexhull);
    process_mesh(itsacuboid);
    process_mesh(itsaheightfield);
    process_mesh(itsaplane);
    process_mesh(itsapolyline);
    process_mesh(itsasegment);
    process_mesh(itsatrimesh);
    process_mesh(itsatriangle);
}
