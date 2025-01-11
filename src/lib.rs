#![feature(ptr_sub_ptr)]

use unity::prelude::*;
use unity::system::List;
use unity::il2cpp::assembly::Il2CppImage;
use skyline::install_hook;
use cobapi::{Event, SystemEvent};
use engage::{
    gamedata::skill::*,
    gamedata::unit::Unit,
    gamedata::*,
    force::ForceType,
    tmpro::TextMeshProUGUI,
};
use skyline::patching::Patch;
use unity::il2cpp::object::Array;

// EquipSkillMenu
// public static InfoUtil.StatusSkill[] GetSkillListForUnitInfo(Unit unit, bool isSkillEquip = False, bool isPack = False, int size = 10) { }
// public class InfoUtil.StatusSkill // TypeDefIndex: 11851
// {
	// Properties
	// public SkillData Data { get; set; }
	// public bool IsActive { get; set; }
	// public SkillData.Categorys Category { get; set; }


#[unity::class("App", "StringBuilder")]
pub struct StringBuilder{
	pub m_ChunkChars: Array<char>, // 0x10
	pub m_ChunkPrevious: Box<StringBuilder>, // 0x18
	pub m_ChunkLength : i32, // 0x20
	pub m_ChunkOffset : i32, // 0x24
	pub m_MaxCapacity : i32 // 0x28
}

#[unity::class("App", "InfoUtil")]
pub struct InfoUtil{
    //pub emptyStringBuilder: StringBuilder
	// private static readonly StringBuilder s_EmptyStringBuilder; // 0x0
}

#[unity::class("App", "InfoUtil_StatusSkill")]
pub struct StatusSkill {
    pub skill_data: Option<&'static SkillData>,
    pub is_active: bool,
    pub category: i32
}

#[unity::class("App", "UnitStatusSetter_SkillSetter")]
// #[derive(Clone, Copy)]
pub struct SkillSetter {
    pub root: *mut GameObject,
    pub icon: *mut Il2CppImage,//8
    pub name: *mut TextMeshProUGUI,//16
    pub godImage: *mut Il2CppImage,
    pub style: *mut i64,
    pub setter: *mut UnitStatusSetter
}

#[unity::class("App", "UnitStatusSetter")]
pub struct UnitStatusSetter {
    pad1: [u8; 0xf8],
    pub skillRoot: *mut i64,// 100
    pad2: [u8; 0x1], //108
    pub list: &'static mut List<SkillSetter>
}

#[unity::class("UnityEngine", "GameObject")]
pub struct GameObject{

}

#[unity::class("UnityEngine", "Quaternion")]
pub struct Quaternion{

}

#[unity::class("UnityEngine", "Vector3")]
pub struct Vector3{
    pub x: f32, // 0x0
	pub y: f32, // 0x4
	pub z: f32 // 0x8
    //more
}

#[unity::class("UnityEngine", "Vector2")]
pub struct Vector2{
    pub x: f32, // 0x0
	pub y: f32, // 0x4
}

#[unity::class("UnityEngine", "RectTransform")]
pub struct RectTransform{
    
}

#[unity::class("UnityEngine", "Transform")]
pub struct Transform{
    
}

#[unity::class("UnityEngine", "Rect")]
pub struct Rect{
   pub xMin: &'static mut f32,
   pub yMin: &'static mut f32,
   pub xSize: &'static mut f32,
   pub ySize: &'static mut f32,
}

// #[unity::hook("App", "UnitStatusSetter", "SetSkill")]
// pub fn set_skill(this: &'static mut UnitStatusSetter, unit: &Unit, _method_info : u64)
// {unsafe{

//     if (this.list.capacity() != 13)
//     {
//         this.list.resize(13);
//     }

//     while (this.list.len() != 13)
//     {
//         let dupeb :&mut Il2CppClass = this.list[2].get_class().clone();
//         let newSetterb = il2cpp::instantiate_class::<SkillSetter>(dupeb).unwrap();
        
//         // alrady set?
//         newSetterb.root = this.skillRoot;

//         let dupet :&mut Il2CppClass = (*this.list[0].name).get_class().clone();
//         let newSettert = il2cpp::instantiate_class::<TextMeshProUGUI>(dupet).unwrap();

//         newSetterb.name = newSettert;
//         // newSetter.setter = &mut *this;

//         skill_setter_init(newSetterb, this, _method_info);

//         this.list.add(newSetterb);
//     }

//     // let val = format!("{} {}", this.list.len(), this.list.capacity());

//     // skyline::error::show_error(
//     //     69,
//     //     "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
//     //     val.as_str(),
//     // );
//     call_original!(this, unit, _method_info);

// }}

#[unity::hook("App", "UnitStatusSetter", "Init")]
pub fn init_skill_ui(this: &'static mut UnitStatusSetter, _method_info : u64)
{
unsafe{
    if (this.list.capacity() != 13)
    {
        this.list.resize(13);
        let err_msg = format!("{}", this.list.capacity());

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }
}

    call_original!(this, _method_info);

}

// #[skyline::from_offset(0x2595CD0)] // use a non templated copy like 0x32EF200
// pub fn clone_go(original: &GameObject, pos: Vector3, rot: Quaternion, parent: Transform, _method_info: u64) -> GameObject;
// doesn't have a gameobject version

#[skyline::from_offset(0x2C4DCF0)]
pub fn get_component_rect(original: &GameObject, strn : &Il2CppString, _method_info: u64) -> &'static RectTransform;

#[skyline::from_offset(0x2F7C3F0)]
pub fn get_transform_rect(original: &RectTransform, _method_info: u64) -> &'static mut Rect;

#[skyline::from_offset(0x2F7AE50)]
pub fn get_ymin(original: &Rect, _method_info: u64) -> f32;

#[skyline::from_offset(0x2F7AE90)]
pub fn get_ymax(original: &Rect, _method_info: u64) -> f32;

#[skyline::from_offset(0x2F7ADB0)]
pub fn set_y(original: &Rect, f : f32, _method_info: u64);

#[skyline::from_offset(0x2F7AEC0)]
pub fn get_height(original: &Rect, _method_info: u64) -> f32;

#[skyline::from_offset(0x2F7DD60)]
pub fn get_parent_rect(original: &RectTransform, _method_info: u64) -> &'static mut Rect;

// #[skyline::from_offset(0x2F7ADC0)]
// pub fn get_xy(original: &Rect, _method_info: u64) -> &'static mut Vector2;

// #[skyline::from_offset(0x2595AF0)]
// pub fn clone_go(original: &GameObject, parent: Transform, staysPos: bool, _method_info: u64) -> GameObject;

// #[skyline::from_offset(0x02c4e880)]
// pub fn get_transform(this: &GameObject, _method_info: u64) -> &'static mut Transform;

#[skyline::from_offset(0x378F9F0)]
pub fn get_local_position(this: &RectTransform, _method_info: u64) -> &'static mut &mut Vector3;

#[skyline::from_offset(0x3797C60)]
pub fn get_vec_item_at_index(this: &Vector3, index: i32, _method_info: u64) -> &'static f32;

// #[skyline::from_offset(0x378FAA0)]
// pub fn set_position(this: &Transform, vec: &Vector3, _method_info: u64);

#[skyline::from_offset(0x378FE20)]
pub fn get_rotation(this: &Transform, _method_info: u64) -> Quaternion;

#[skyline::from_offset(0x37909F0)]
pub fn get_parent(this: &Transform, _method_info: u64) -> Transform;

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2486FC0)]
pub fn skillarray_exists(this: &SkillArray, _method_info: u64) -> bool;

#[unity::from_offset("App", "Unit", "get_EquipSkill")]
pub fn get_equiped_skills(this: &Unit, _method_info: u64) -> Option<&SkillArray>;

// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_IsActive")]
#[skyline::from_offset(0x1FC7310)]
pub fn set_is_active(this: &StatusSkill, active: bool, _method_info: u64);

// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_Category")]
#[skyline::from_offset(0x1FC7330)]
pub fn set_category(this: &StatusSkill, cat: i32, _method_info: u64);

// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_Data")]
#[skyline::from_offset(0x1FC72F0)]
pub fn set_data(this: &StatusSkill, value: Option<&SkillData>, _method_info: u64);

#[skyline::from_offset(0x1B59230)]
pub fn skill_setter_init(this: &SkillSetter, status: &mut UnitStatusSetter, _method_info: u64);

// #[skyline::from_offset(0x1B59230)]
// pub fn skill_setter_ctor(this: &SkillSetter, _method_info: u64);


// pub fn infoutil_getskilllistforunitinfo(unit: &Unit, isskillequip: bool, ispack: bool, size: i32 , method_info: OptionalMethod) -> &Array<Option<&'static StatusSkill>>
    
#[unity::hook("App", "InfoUtil", "GetSkillListForUnitInfo")]
pub fn get_skill_list(unit: Option<&Unit>, is_equip: bool, is_pack: bool, mut size: i32, _method_info : u64) -> &'static mut Array<&'static StatusSkill>
{unsafe{
    
    size = 13;
    let mut original: &'static mut Array<&'static StatusSkill> = call_original!(unit, true, is_pack, size, _method_info);

    // if original.len() >= 10 // only used on equip screen
    // {
        if let Some(person) = unit
        {
            // ignore foe
            if person.person.get_asset_force() == ForceType::Player as i32
            {
                if let Some(equips) = get_equiped_skills(person, _method_info)
                {
                    // make room for the new equip skill slots
                    for x in (5..original.len()).rev()
                    {
                        original[x] = original[x-3];
                    }
                    for x in 2..5
                    {
                        if let Some(equipedSkill) = equips[x as usize].get_skill()
                        {
                            let dupet :&mut Il2CppClass = (original[x]).get_class().clone();
                            let newt = il2cpp::instantiate_class::<StatusSkill>(dupet).unwrap();
                            original[x as usize] = newt;
                            set_category(original[x as usize], 11, _method_info); 
                            let sid = equipedSkill.sid.get_string().unwrap_or("".to_string());
                            if sid == "SID_無し" || sid == "無し" || sid == ""
                            {
                                set_data(original[x as usize], None, _method_info);
                                set_is_active(original[x as usize], false, _method_info);
                            }
                            else
                            {
                                set_data(original[x as usize], Some(equipedSkill), _method_info);
                                set_is_active(original[x as usize], true, _method_info);
                            }
                        }
                    }
                }
            }
        }
    // }
      
    return original;
}}







#[skyline::main(name = "moreSlots")]
pub fn main() {
    println!("No Unique Weapons plugin loaded");

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    // chekcs in add skill for max count (5 now)
    // 01a35fd0 && 01a35ff
    Patch::in_text(0x01a35fd0).bytes(&[0x1f, 0x11, 0x00, 0x71]).unwrap();
    Patch::in_text(0x01a35ff8).bytes(&[0x1f, 0x11, 0x00, 0x71]).unwrap();

    // set color change count to 5
    // 0249b3f8 3f 11 00 71
    // never called?
    // Patch::in_text(0x0249b3f8).bytes(&[0x3f, 0x11, 0x00, 0x71]).unwrap();zx


    // remove auto return on inheritance update thingy
    Patch::in_text(0x0249b394).bytes(&[0xC0, 0x01, 0x00, 0x54]).unwrap();

    // make eskill list only 5 items in the UI
    // PUT BACK LATER THIS SHOWS US WHATS WRONG WITH THE PERCEPTION THING
    Patch::in_text(0x02499c8c).bytes(&[0x37, 0x00, 0x00, 0x14]).unwrap();

    // stop the auto sizing
    //      7101c699d8 13 00 00 14     b          LAB_7101c69a24
    // Patch::in_text(0x01c699d8).bytes(&[0x13, 0x00, 0x00, 0x14]).unwrap();
    
    // remove the 2nd index skip when 1st index is empty 
    // from the equip menu
    Patch::in_text(0x0249d318).bytes(&[0x18, 0x00, 0x00, 0x14]).unwrap();

    install_hook!(get_skill_list);
    // install_hook!(init_skill_ui);
}