"In Sui, a package is a collection of Move modules. 
Each module is an independent unit of code defining specific functions and logic. 
A package can contain multiple modules that together define a complex application or smart contract."

To create a new Move package, we can use the "sui move new package_name" command.

Package Structure

On-chain, a package consists of multiple modules—these modules are independent scopes containing functions, types, and other items.

Local Package Structure

<img width="645" height="182" alt="image" src="https://github.com/user-attachments/assets/88b69ce6-f4f4-4b89-9488-94fdd8e6da61" />

Publishing a Package

During development, the package address is set to 0x0. Once published to the blockchain, it will receive a unique address containing the bytecode of its modules. 
Once published, the package becomes immutable and its functions can be called by sending transactions, for example, 0x1::ModuleName::function_name.
