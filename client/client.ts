import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Tienda;

const owner = provider.wallet;

(async () => {

  // pda de la tienda
  const [tiendaPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("tienda"),
      owner.publicKey.toBuffer()
    ],
    program.programId
  );

  // crear tienda
  await program.methods
    .crearTienda("tienda los medina")
    .accounts({
      owner: owner.publicKey,
      tienda: tiendaPda,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();

  console.log("tienda creada");

  // agregar producto
  await program.methods
    .agregarProducto("arroz", 30)
    .accounts({
      owner: owner.publicKey,
      tienda: tiendaPda
    })
    .rpc();

  console.log("producto agregado");

  // ver productos
  await program.methods
    .verProductos()
    .accounts({
      owner: owner.publicKey,
      tienda: tiendaPda
    })
    .rpc();

})();