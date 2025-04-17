# 🛍️ Rust Store Management System

A modular command-line store management system built in Rust. This project demonstrates how to design an extensible application using traits, generics, and clean architecture. The system supports both **Admin** and **Buyer** roles with separate operations and permissions.

---

## 📦 Features

- **Admin Panel:**
  - Add new products to the store
  - View all listed products
  - Update product details
  - Delete products
  - View buyer orders

- **Buyer Panel:**
  - View available products
  - Select and purchase products
  - View selected products
  - Delete or update selected products

- **Core Concepts:**
  - Trait-based system operation handler
  - Generic inventory and product handling
  - Fully CLI-interactive

---

## 🗂️ Project Structure

```bash
src/
├── main.rs                      # Entry point
├── authentication.rs            # Handles login/user type selection
├── operation/
│   ├── inventory.rs             # Generic inventory logic with trait
│   ├── info_model.rs            # Shared product info trait
│   └── system_operation.rs      # Generic system operation handler trait
├── store/
│   ├── admin_system.rs          # Admin system logic
│   ├── store_model.rs           # Store-side product model
│   ├── store_indicator.rs       # Admin menu indicator
│   ├── view_products.rs         # View products logic
│   └── delete_product.rs        # Delete product from inventory
├── users/
│   ├── buyer_system.rs          # Buyer system logic
│   ├── buyer_model.rs           # Buyer-side product model
│   ├── user_indicator.rs        # Buyer menu indicator
│   └── buy_product.rs           # Buying logic
├── company/
│   └── company_indicator.rs     # Role selection (Admin/User)
└── utils.rs                     # Shared utilities (e.g., input handling)
