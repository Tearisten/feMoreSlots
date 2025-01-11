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
}

#[unity::class("App", "InfoUtil_StatusSkill")]
pub struct StatusSkill {
    pub skill_data: Option<&'static SkillData>,
    pub is_active: bool,
    pub category: i32
}

#[unity::class("App", "UnitStatusSetter_SkillSetter")]
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

    
#[unity::hook("App", "InfoUtil", "GetSkillListForUnitInfo")]
pub fn get_skill_list(unit: Option<&Unit>, is_equip: bool, is_pack: bool, mut size: i32, _method_info : u64) -> &'static mut Array<&'static StatusSkill>
{unsafe{
    
    size = 13;
    let mut original: &'static mut Array<&'static StatusSkill> = call_original!(unit, true, false, size, _method_info);

    if is_pack || is_equip
    {
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
    }
      
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
    Patch::in_text(0x01a35fd0).bytes(&[0x1f, 0x11, 0x00, 0x71]).unwrap();
    Patch::in_text(0x01a35ff8).bytes(&[0x1f, 0x11, 0x00, 0x71]).unwrap();

    // remove auto return on inheritance update thingy
    Patch::in_text(0x0249b394).bytes(&[0xC0, 0x01, 0x00, 0x54]).unwrap();

    // make eskill list only 5 items in the UI
    Patch::in_text(0x02499c8c).bytes(&[0x37, 0x00, 0x00, 0x14]).unwrap();

    // remove the 2nd index skip when 1st index is empty 
    // from the equip menu
    Patch::in_text(0x0249d318).bytes(&[0x18, 0x00, 0x00, 0x14]).unwrap();

    install_hook!(get_skill_list);
}