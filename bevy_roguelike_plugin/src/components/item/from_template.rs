use super::MutableQuality;
use super::Quality;
use crate::components::*;
use crate::resources::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_inventory_ui::UiRenderInfo;
use rand::prelude::*;


pub fn spawn_item(
    ecmd: &mut EntityCommands,
    asset_server: AssetServer,
    template: &ItemTemplate,
    quality: &Quality,
    rng: &mut StdRng,
) {
    ecmd.insert(quality.clone());
    match template {
        ItemTemplate::Weapon(Weapon { render, damage }) => {
            ecmd.insert(RogueItemType::MainHand);
            insert_render(ecmd, asset_server, render);
            ecmd.insert(damage.mutate(quality, rng));
        }
        ItemTemplate::Shield(Shield {
            render,
            protection,
            block,
        }) => {
            ecmd.insert(RogueItemType::OffHand);
            insert_render(ecmd, asset_server, render);
            ecmd.insert(protection.mutate(quality, rng))
                .insert(block.mutate(quality, rng));
        }
        ItemTemplate::Helm(Helm {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(RogueItemType::Head);
            insert_render(ecmd, asset_server, render);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
        ItemTemplate::Armor(Armor {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(RogueItemType::Body);
            insert_render(ecmd, asset_server, render);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
        ItemTemplate::Boots(Boots {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(RogueItemType::Feet);
            insert_render(ecmd, asset_server, render);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
        ItemTemplate::Amulet(Amulet {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(RogueItemType::Neck);
            insert_render(ecmd, asset_server, render);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
        ItemTemplate::Ring(Ring {
            render,
            defense,
            enchantment,
        }) => {
            ecmd.insert(RogueItemType::Finger);
            insert_render(ecmd, asset_server, render);
            insert_defense(ecmd, defense, quality, rng);
            insert_enchantment(ecmd, enchantment, quality, rng);
        }
    }
}

fn insert_defense(
    ecmd: &mut EntityCommands,
    defense: &ItemDefense,
    quality: &Quality,
    rng: &mut StdRng,
) {
    if let Some(prot) = defense.protection.clone() {
        ecmd.insert(prot.mutate(quality, rng));
    }
    if let Some(res) = defense.resistance.clone() {
        ecmd.insert(res.mutate(quality, rng));
    }
}
fn insert_enchantment(
    ecmd: &mut EntityCommands,
    enchantment: &ItemEnchantment,
    quality: &Quality,
    rng: &mut StdRng,
) {
    if let Some(attributes) = enchantment.attributes.clone() {
        ecmd.insert(attributes.mutate(quality, rng));
    }
}

fn insert_render(ecmd: &mut EntityCommands, asset_server: AssetServer, render: &ItemRenderInfo) {
    let texture = asset_server.load(render.texture_path.as_str());
    ecmd.insert(Name::new(render.name.clone()))
        .insert(UiRenderInfo {
            image: texture.clone().into(),
        })
        .insert(RenderInfo {
            texture,
            cosmetic_textures: vec![],
            z: 1.,
        });
    if let Some(path_equiped) = render.texture_equiped_path.clone() {
        ecmd.insert(RenderInfoEquiped {
            texture: asset_server.load(path_equiped.as_str()),
            z: 4.,
        });
    }
}
