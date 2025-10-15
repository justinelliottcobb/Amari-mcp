#!/usr/bin/env python3
"""
Example MCP client usage for Amari mathematical computing server.

This demonstrates how to use the Amari MCP server from Python.
"""

import asyncio
import json
import math
from typing import Any, Dict, List

# Note: This is pseudocode - replace with actual MCP client library
class MCPClient:
    def __init__(self, server_url: str):
        self.server_url = server_url

    async def call_tool(self, tool_name: str, params: Dict[str, Any]) -> Dict[str, Any]:
        # This would be the actual MCP client implementation
        print(f"Calling tool: {tool_name} with params: {params}")
        return {"success": True, "result": "placeholder"}

async def geometric_algebra_examples(client: MCPClient):
    """Examples of geometric algebra operations."""
    print("🧮 Geometric Algebra Examples")
    print("=" * 40)

    # Create a multivector in 3D Euclidean space
    print("\n1. Creating a multivector:")
    mv_result = await client.call_tool("create_multivector", {
        "coefficients": [1.0, 0.5, 0.3, 0.2, 0.1, 0.15, 0.25, 0.05],
        "signature": [3, 0, 0]
    })
    print(f"   Result: {json.dumps(mv_result, indent=2)}")

    # Geometric product of two multivectors
    print("\n2. Geometric product:")
    product_result = await client.call_tool("geometric_product", {
        "a": [1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],  # 1 + e₁
        "b": [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],  # e₂
        "signature": [3, 0, 0]
    })
    print(f"   (1 + e₁) * e₂ = {json.dumps(product_result, indent=2)}")

    # Rotor rotation
    print("\n3. Rotor rotation (90° around Z-axis):")
    rotation_result = await client.call_tool("rotor_rotation", {
        "vector": [1.0, 0.0, 0.0],  # X-axis vector
        "axis": [0.0, 0.0, 1.0],    # Z-axis
        "angle": math.pi / 2        # 90 degrees
    })
    print(f"   Rotating [1,0,0] by 90° around Z: {json.dumps(rotation_result, indent=2)}")

async def tropical_algebra_examples(client: MCPClient):
    """Examples of tropical algebra operations."""
    print("\n\n🏝️ Tropical Algebra Examples")
    print("=" * 40)

    # Tropical matrix multiplication
    print("\n1. Tropical matrix multiplication:")
    matrix_result = await client.call_tool("tropical_matrix_multiply", {
        "matrix_a": [
            [0, 3, float('inf')],
            [2, 0, 1],
            [float('inf'), 4, 0]
        ],
        "matrix_b": [
            [0, 1],
            [2, 0],
            [3, 2]
        ]
    })
    print(f"   Result: {json.dumps(matrix_result, indent=2)}")

    # Shortest path computation
    print("\n2. Shortest path in graph:")
    # Graph adjacency matrix with edge weights
    graph = [
        [0, 2, None, 1],        # Node 0: edges to 1(cost=2), 3(cost=1)
        [None, 0, 3, 2],        # Node 1: edges to 2(cost=3), 3(cost=2)
        [None, None, 0, 1],     # Node 2: edge to 3(cost=1)
        [None, None, None, 0]   # Node 3: no outgoing edges
    ]

    path_result = await client.call_tool("shortest_path", {
        "adjacency_matrix": graph,
        "source": 0,
        "target": 2
    })
    print(f"   Shortest path from 0 to 2: {json.dumps(path_result, indent=2)}")

async def autodiff_examples(client: MCPClient):
    """Examples of automatic differentiation."""
    print("\n\n🔄 Automatic Differentiation Examples")
    print("=" * 40)

    # Compute gradient
    print("\n1. Computing gradient of f(x,y) = x²y + sin(x):")
    gradient_result = await client.call_tool("compute_gradient", {
        "expression": "x^2 * y + sin(x)",
        "variables": ["x", "y"],
        "values": [1.0, 2.0]
    })
    print(f"   ∇f(1,2) = {json.dumps(gradient_result, indent=2)}")

async def cellular_automata_examples(client: MCPClient):
    """Examples of cellular automata evolution."""
    print("\n\n🔬 Cellular Automata Examples")
    print("=" * 40)

    # Simple 4x4 grid with geometric rule
    print("\n1. Evolving 4x4 geometric CA:")

    # Initial state: central activation
    initial_state = []
    for i in range(16):  # 4x4 grid
        if i == 5 or i == 6 or i == 9 or i == 10:  # Center 2x2
            # Active cell with multivector [scalar, e1, e2, e3, e12, e13, e23, e123]
            initial_state.append([1.0, 0.5, 0.3, 0.2, 0.1, 0.15, 0.25, 0.05])
        else:
            # Inactive cell
            initial_state.append([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])

    ca_result = await client.call_tool("ca_evolution", {
        "initial_state": initial_state,
        "rule": "geometric",
        "steps": 5,
        "grid_width": 4,
        "grid_height": 4
    })
    print(f"   CA evolution result: {json.dumps(ca_result, indent=2)}")

async def info_geometry_examples(client: MCPClient):
    """Examples of information geometry computations."""
    print("\n\n📊 Information Geometry Examples")
    print("=" * 40)

    # Fisher information for Gaussian distribution
    print("\n1. Fisher information matrix for Gaussian:")
    fisher_result = await client.call_tool("fisher_information", {
        "distribution": "gaussian",
        "parameters": [0.0, 1.0],  # mean=0, variance=1
        "data": [0.1, -0.5, 0.8, -0.2, 1.1, 0.3, -0.7, 0.9]
    })
    print(f"   Fisher matrix: {json.dumps(fisher_result, indent=2)}")

async def gpu_examples(client: MCPClient):
    """Examples of GPU-accelerated computations."""
    print("\n\n🚀 GPU Acceleration Examples")
    print("=" * 40)

    # Batch geometric products on GPU
    print("\n1. Batch geometric products on GPU:")

    # Generate batch data
    batch_data = []
    for i in range(1000):
        a = [1.0, float(i % 4), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        b = [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        batch_data.append({"a": a, "b": b})

    gpu_result = await client.call_tool("gpu_batch_compute", {
        "operation": "geometric_product",
        "data": batch_data,
        "batch_size": 256
    })
    print(f"   GPU batch result: {json.dumps(gpu_result, indent=2)}")

async def database_examples(client: MCPClient):
    """Examples of database operations."""
    print("\n\n💾 Database Examples")
    print("=" * 40)

    # Save computation result
    print("\n1. Saving computation to database:")
    save_result = await client.call_tool("save_computation", {
        "name": "example_geometric_product",
        "type": "geometric_algebra",
        "result": {
            "coefficients": [0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            "operation": "geometric_product"
        },
        "metadata": {
            "signature": [3, 0, 0],
            "timestamp": "2024-01-01T12:00:00Z"
        }
    })
    print(f"   Save result: {json.dumps(save_result, indent=2)}")

    # Load computation result
    print("\n2. Loading computation from database:")
    load_result = await client.call_tool("load_computation", {
        "name": "example_geometric_product"
    })
    print(f"   Load result: {json.dumps(load_result, indent=2)}")

async def main():
    """Run all examples."""
    print("🎯 Amari MCP Server Examples")
    print("=" * 50)

    # Initialize MCP client
    client = MCPClient("http://localhost:3000")

    try:
        await geometric_algebra_examples(client)
        await tropical_algebra_examples(client)
        await autodiff_examples(client)
        await cellular_automata_examples(client)
        await info_geometry_examples(client)
        await gpu_examples(client)
        await database_examples(client)

        print("\n\n✅ All examples completed!")
        print("\nTo run the Amari MCP server:")
        print("  cargo run --release --features gpu,database -- --gpu --database-url $DATABASE_URL")

    except Exception as e:
        print(f"❌ Error running examples: {e}")

if __name__ == "__main__":
    asyncio.run(main())