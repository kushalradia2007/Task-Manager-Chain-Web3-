# ⛓️ Task Manager Chain

## 📑 Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)

---

## 🏷️ Project Title
**Task Manager Chain**

## 📖 Project Description
Task Manager Chain is a decentralized application built on the Stellar network using the Soroban smart contract SDK. It serves as a secure, immutable ledger for task management. Users can generate tasks with custom descriptions, store them permanently on-chain, and seamlessly update their status to "completed" once the work is finalized. 

## 🔭 Project Vision
The vision of Task Manager Chain is to bring absolute transparency and accountability to task tracking. By moving to-do lists and operational logs to the blockchain, we ensure that task histories cannot be retroactively altered, deleted, or manipulated by a centralized administrator, creating a trustless environment for personal productivity and team collaboration.

## ✨ Key Features
* **On-Chain Task Creation:** Users can register new tasks directly to the ledger with descriptive text.
* **Immutable State Updates:** Active tasks can be irreversibly marked as `completed`.
* **Sequential Indexing:** Every task generated on the chain is automatically assigned a unique, sequential integer ID for easy querying.
* **Open Querying:** Anyone can retrieve real-time details and the current completion status of a specific task using its unique ID.

## 🚀 Future Scope
* **Wallet-Based Access Control:** Implement strict authentication so that only the original creator of a task (verified via their Stellar public key) has the authority to mark it as completed.
* **Deadlines and Timestamps:** Integrate ledger timestamps to enforce due dates and track exactly when a task was finished.
* **Delegation:** Allow users to assign a specific task to another wallet address on the network.
* **Frontend Dashboard:** Develop a user-friendly Web3 interface using React and `freighter-api` to interact smoothly with the Task Manager Chain smart contract.

## 📜 Contract Details
**Contract ID:** `CBE4JVLKJE7Q6U3OWMDVST4VE2A23JK5AJY4VC4O3GQTS6TBQKJVDUM7`

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/6a4db4be-6765-40b1-883f-ca0c40f54277" />
