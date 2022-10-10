use std::default;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tracer {
    use super::*;


    pub fn operate(ctx: Context<Operate>) -> Result<()> {
        let tuple_1 = ctx.accounts.eqn.element_1.as_ref().unwrap_or_default();
        let tuple_2= ctx.accounts.eqn.element_2.as_ref().unwrap_or_default();
        let result = match &ctx.accounts.eqn.operation {
            &Operation::Addition => {
                Tuple {
                    x: tuple_1.x + tuple_2.x,
                    y: tuple_1.y + tuple_2.y,
                    z: tuple_1.z + tuple_2.z,
                    t: crate::TupleType::Vector
                    }
                },

            _ => {
                Tuple {
                    x: tuple_1.x + tuple_2.x,
                    y: tuple_1.y + tuple_2.y,
                    z: tuple_1.z + tuple_2.z,
                    t: crate::TupleType::Vector
                    }
                },   
        };
        ctx.accounts.eqn.result = Some(result);
        Ok(())
    }

    pub fn new(ctx: Context<EqnSetup>)
    -> Result<()> {
        let eqn = &mut ctx.accounts.eqn;
        eqn.new_eqn()
    }

}



#[derive(Accounts)]
pub struct Operate<'info> {
    pub eqn: Account<'info, Eqn>,

}

#[derive(Accounts)]
pub struct EqnSetup<'info> {
    #[account(init, payer = operator, space = 8 + 50)]
    pub eqn: Account<'info, Eqn>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Eqn {
    element_1: Option<Tuple>, // 1 + 4
    element_2: Option<Tuple>,
    operation: Operation, // 1 
    result: Option<Tuple>, // 1 + 1

}

impl Eqn {
    pub fn new_eqn(&mut self) 
    -> Result<()> {
        self.element_1 = None;
        self.element_2 = None;
        self.operation = crate::Operation::Addition;
        self.result = None;
        Ok(())
    }
    pub fn set_element_1 (&mut self, incoming: Tuple) 
    -> Result<()>{
        self.element_1 = Some(incoming);
        Ok(())
    }

    pub fn set_element_2 (&mut self, incoming: Tuple) 
    -> Result<()>{
        self.element_2 = Some(incoming);
        Ok(())
    }

    pub fn set_operation (&mut self, incoming: Operation) 
    -> Result<()>{
        self.operation = incoming;
        Ok(())
    }
}


#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy
)]
pub enum Operation {
    Addition,
    Subtraction,
}

#[derive(
    AnchorSerialize, AnchorDeserialize, Clone
)]
pub enum TupleType {
    Vector,
    Point,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Tuple {
    x: u8,
    y: u8,
    z: u8,
    t: TupleType
}

impl Default for &Tuple {
    fn default() -> Self {
        { &Tuple {
            x: 0,
            y: 0,
            z: 0,
            t: crate::TupleType::Vector
        }}
    }
}