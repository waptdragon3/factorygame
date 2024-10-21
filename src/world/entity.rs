#[derive(Debug, Copy, Clone, Default)]
struct CraftingBehavior;


#[derive(Debug, Copy, Clone, Default)]
struct BeltBehavior;


#[derive(Debug, Copy, Clone, Default)]
pub enum Behavior {
    #[default] None,
    CraftingMachine(CraftingBehavior),
    TransportBelt(BeltBehavior),

}

#[derive(Debug, Copy, Clone, Default)]
pub struct Inventory;


#[derive(Debug, Copy, Clone, Default)]
pub struct Blueprint;



#[derive(Debug, Copy, Clone, Default)]
pub struct EID (u32);
#[derive(Debug, Copy, Clone, Default)]
pub struct Entity {
    _id: EID,
    pub inventory: Inventory,
    pub behavior: Behavior,
    pub bp: Blueprint,
}