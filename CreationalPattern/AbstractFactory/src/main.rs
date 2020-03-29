/*  
*   Core
*/

trait Image {
    fn get_image(&self) -> String;
}

trait Factory<P: Image> {
    fn create_container(&self) -> P;
}

/*  
*   TensorflowContainer
*/

struct TensorflowImage;
struct TensorflowContainer;

impl Image for TensorflowImage {
    fn get_image(&self) -> String {
        return String::from("tensorflow/tensorflow");
    }
}

impl Factory<TensorflowImage> for TensorflowContainer {
    fn create_container(&self) -> TensorflowImage {
        println!("Success : {}", TensorflowImage.get_image());
        return TensorflowImage;
    }
}

/*  
*   RContainer
*/

struct RImage;
struct RContainer;

impl Image for RImage {
    fn get_image(&self) -> String {
        return String::from("tensorflow/tensorflow");
    }
}

impl Factory<RImage> for RContainer {
    fn create_container(&self) -> RImage {
        println!("Success : {}", RImage.get_image());
        return RImage;
    }
}

fn main() {
    let tensorflow = TensorflowContainer;
    let r = RContainer;

    tensorflow.create_container();
    r.create_container();
}
