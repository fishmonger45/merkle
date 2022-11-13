use sha2::Digest;
use util::pow2;

mod util;

type Block = Vec<u8>;
type Hash = Vec<u8>;

#[derive(Default, Debug)]
pub struct MerkleTree {
    nodes: Vec<Hash>,
}

impl MerkleTree {
    pub fn verify(root: &Hash, leaves: Vec<Block>) -> bool {
        *Self::build(leaves).root() == *root
    }

    pub fn build(leaves: Vec<Block>) -> Self {
        // balanced
        assert!(pow2(leaves.len()));
        let mut hashed_levels: Vec<Vec<Hash>> = vec![leaves.iter().map(Self::hash_leaf).collect()];
        let mut last_level = &hashed_levels[0];
        let depth = (leaves.len() as f64).log2() as usize;

        for _ in 0..depth {
            let mut next_level = vec![Self::hash_level(last_level)];
            hashed_levels.append(&mut next_level);
            last_level = &hashed_levels[hashed_levels.len() - 1]
        }

        Self {
            nodes: hashed_levels.into_iter().flatten().collect(),
        }
    }

    pub fn root(&self) -> &Hash {
        assert!(self.nodes.len() > 0, "empty merkle tree");
        &self.nodes[0]
    }

    fn hash_concat(a: &Hash, b: &Hash) -> Hash {
        let b = a.iter().chain(b).copied().collect();
        Self::hash_leaf(&b)
    }

    fn hash_level(level: &Vec<Hash>) -> Vec<Hash> {
        assert!(pow2(level.len()));
        level
            .chunks(2)
            .map(|p| Self::hash_concat(&p[0], &p[1]))
            .collect()
    }

    fn hash_leaf(leaf: &Block) -> Hash {
        sha2::Sha256::digest(leaf).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data: Vec<Vec<u8>> = vec!["protein", "powder", "is", "great"]
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();

        let mt = MerkleTree::build(data.clone());
        assert_eq!(&mt.nodes[0], mt.root());
        assert!(MerkleTree::verify(mt.root(), data));
        assert_eq!(&mt.nodes.len(), &7usize);
    }
}
