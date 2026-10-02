import os
import random

def generate_sample_yaml(path: str = "output/sample.yaml", size_mb: int = 100) -> str:
    target_size = size_mb * 1024 * 1024
    
    os.makedirs(os.path.dirname(path), exist_ok=True)

    with open(path, "w", encoding="utf-8") as f:
        f.write("items:\n")
        i = 0
        while f.tell() < target_size:
            f.write(f"  - id: {i}\n")
            f.write(f"    name: item_{i}\n")
            f.write(f"    bool: {random.choice(['true', 'false'])}\n")
            f.write(f"    price: {random.uniform(0.01, 9999.99):.2f}\n")
            f.write(f"    rating: {random.uniform(0.0, 5.0):.1f}\n")
            f.write(f"    emoji: '{''.join(random.sample(['⌨️', '🖱️', '📱'], 1))}'\n")
            f.write(f"    unicode: 'オージェイソンが明らかに勝った'\n")
            f.write(f"    description: This is a sample description for item number {i}. "
                    f"It exists only to increase the file size for testing memory usage.\n")
            f.write(f"    values: [{', '.join(str(j) for j in range(10))}]\n")
            i += 1

    return path


if __name__ == "__main__":
    generate_sample_yaml()