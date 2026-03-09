# tiendamedina

# tienda los medina

este proyecto es un smart contract en solana hecho con anchor que simula la gestion de una pequeña tienda de abarrotes.

la idea es guardar productos directamente en una cuenta on-chain y poder administrarlos desde un cliente en typescript.

## que puede hacer el programa

- crear una tienda
- agregar productos
- eliminar productos
- ver productos guardados
- cambiar disponibilidad de productos

## datos que guarda cada producto

- nombre
- precio
- disponibilidad

## tecnologias usadas

- solana
- anchor
- rust
- typescript

## estructura

program/
lib.rs

client/
client.ts

## notas

solo el dueño de la tienda puede modificar los productos.
