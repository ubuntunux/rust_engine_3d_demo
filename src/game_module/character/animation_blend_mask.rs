use std::collections::HashMap;

pub struct AnimationBlendMasks {
    pub _upper_animation_mask: HashMap<String, f32>
}

impl AnimationBlendMasks {
    pub fn create_animation_blend_maks() -> AnimationBlendMasks {
        AnimationBlendMasks {
            _upper_animation_mask: AnimationBlendMasks::get_upper_animation_mask()
        }
    }

    pub fn get_upper_animation_mask() -> HashMap<String, f32> {
        let mut animation_mask: HashMap<String, f32> = HashMap::new();
        animation_mask.insert(String::from("mixamorig:Spine"), 1.0);
        animation_mask.insert(String::from("mixamorig:Spine1"), 1.0);
        animation_mask.insert(String::from("mixamorig:Spine2"), 1.0);
        animation_mask.insert(String::from("mixamorig:Neck"), 1.0);
        animation_mask.insert(String::from("mixamorig:Head"), 1.0);
        animation_mask.insert(String::from("mixamorig:HeadTop_End"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftEye"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightEye"), 1.0);
        animation_mask.insert(String::from("mixamorig:Hair1"), 1.0);
        animation_mask.insert(String::from("mixamorig:Hair2"), 1.0);
        animation_mask.insert(String::from("mixamorig:Hair3"), 1.0);
        animation_mask.insert(String::from("mixamorig:Hair4"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftShoulder"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftArm"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftForeArm"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHand"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandThumb1"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandThumb2"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandThumb3"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandThumb4"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandIndex1"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandIndex2"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandIndex3"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandIndex4"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandMiddle1"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandMiddle2"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandMiddle3"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandMiddle4"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandRing1"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandRing2"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandRing3"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandRing4"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandPinky1"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandPinky2"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandPinky3"), 1.0);
        animation_mask.insert(String::from("mixamorig:LeftHandPinky4"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightShoulder"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightArm"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightForeArm"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHand"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandThumb1"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandThumb2"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandThumb3"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandThumb4"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandIndex1"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandIndex2"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandIndex3"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandIndex4"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandMiddle1"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandMiddle2"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandMiddle3"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandMiddle4"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandRing1"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandRing2"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandRing3"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandRing4"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandPinky1"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandPinky2"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandPinky3"), 1.0);
        animation_mask.insert(String::from("mixamorig:RightHandPinky4"), 1.0);
        animation_mask.insert(String::from("mixamorig:Weapon"), 1.0);
        animation_mask
    }
}