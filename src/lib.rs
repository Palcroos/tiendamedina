use anchor_lang::prelude::*;

// id del programa (solana playground lo cambia al compilar)
declare_id!("D8hfDt1HY3kPaPbdUk5aQzm94qyc4GrHV98FwR729jmQ");

#[program]
pub mod tienda {
    use super::*;

    ////////////////////// crear tienda //////////////////////
    // aqui creo la cuenta principal donde se guarda la tienda
    pub fn crear_tienda(ctx: Context<NuevaTienda>, nombre: String) -> Result<()> {

        // guardo quien es el dueño
        let owner = ctx.accounts.owner.key();

        // empiezo con lista vacia de productos
        let productos: Vec<Producto> = Vec::new();

        ctx.accounts.tienda.set_inner(Tienda {
            owner,
            nombre,
            productos,
        });

        msg!("tienda creada correctamente");

        Ok(())
    }

    ////////////////////// agregar producto //////////////////////
    // agrego un producto nuevo a la tienda
    pub fn agregar_producto(
        ctx: Context<ModificarTienda>,
        nombre: String,
        precio: u16,
    ) -> Result<()> {

        // verifico que el que ejecuta sea el dueño
        require!(
            ctx.accounts.tienda.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let producto = Producto {
            nombre,
            precio,
            disponible: true,
        };

        ctx.accounts.tienda.productos.push(producto);

        msg!("producto agregado");

        Ok(())
    }

    ////////////////////// eliminar producto //////////////////////
    // busco un producto por nombre y lo elimino
    pub fn eliminar_producto(
        ctx: Context<ModificarTienda>,
        nombre: String,
    ) -> Result<()> {

        require!(
            ctx.accounts.tienda.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut ctx.accounts.tienda.productos;

        for i in 0..productos.len() {
            if productos[i].nombre == nombre {
                productos.remove(i);
                msg!("producto eliminado");
                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }

    ////////////////////// ver productos //////////////////////
    // solo imprime los productos en logs
    pub fn ver_productos(ctx: Context<ModificarTienda>) -> Result<()> {

        require!(
            ctx.accounts.tienda.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("lista de productos: {:#?}", ctx.accounts.tienda.productos);

        Ok(())
    }

    ////////////////////// cambiar disponibilidad //////////////////////
    // cambia si el producto esta disponible o no
    pub fn alternar_estado(
        ctx: Context<ModificarTienda>,
        nombre: String,
    ) -> Result<()> {

        require!(
            ctx.accounts.tienda.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut ctx.accounts.tienda.productos;

        for i in 0..productos.len() {

            if productos[i].nombre == nombre {

                let estado_actual = productos[i].disponible;

                productos[i].disponible = !estado_actual;

                msg!("estado cambiado");

                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }
}

//////////////////// errores ////////////////////

#[error_code]
pub enum Errores {

    #[msg("no eres el dueño")]
    NoEresElOwner,

    #[msg("producto no encontrado")]
    ProductoNoExiste,
}

//////////////////// cuenta tienda ////////////////////

#[account]
#[derive(InitSpace)]
pub struct Tienda {

    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(15)]
    pub productos: Vec<Producto>,
}

//////////////////// struct producto ////////////////////

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug, PartialEq)]
pub struct Producto {

    #[max_len(60)]
    pub nombre: String,

    pub precio: u16,

    pub disponible: bool,
}

//////////////////// contextos ////////////////////

#[derive(Accounts)]
pub struct NuevaTienda<'info> {

    // wallet que paga la transaccion
    #[account(mut)]
    pub owner: Signer<'info>,

    // cuenta donde se guarda la tienda
    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarTienda<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}