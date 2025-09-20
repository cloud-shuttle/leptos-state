//! Feature tracking for competitive analysis
//! Following ADR-006: Competitive Analysis Strategy

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a competitor's feature set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorFeatures {
    pub name: String,
    pub features: Vec<String>,
    pub performance: PerformanceMetrics,
    pub last_updated: String,
}

/// Performance metrics for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub bundle_size: String,
    pub initialization_time: String,
    pub update_frequency: String,
}

/// Feature tracker for competitive analysis
#[derive(Debug, Clone)]
pub struct FeatureTracker {
    competitors: HashMap<String, CompetitorFeatures>,
    our_features: Vec<String>,
}

impl FeatureTracker {
    /// Create a new feature tracker
    pub fn new() -> Self {
        Self {
            competitors: HashMap::new(),
            our_features: vec![
                "state-management".to_string(),
                "state-machines".to_string(),
                "reactive-updates".to_string(),
                "devtools".to_string(),
                "persistence".to_string(),
                "middleware".to_string(),
                "time-travel".to_string(),
                "hot-reload".to_string(),
                "ssr-support".to_string(),
                "typescript-support".to_string(),
                "performance".to_string(),
                "code-generation".to_string(),
                "visualization".to_string(),
                "testing-utilities".to_string(),
                "migration-tools".to_string(),
                "bundle-optimization".to_string(),
                "async-handling".to_string(),
                "error-recovery".to_string(),
                "analytics".to_string(),
                "monitoring".to_string(),
            ],
        }
    }

    /// Add a competitor
    pub fn add_competitor(&mut self, competitor: CompetitorFeatures) {
        self.competitors.insert(competitor.name.clone(), competitor);
    }

    /// Get feature gap analysis
    pub fn get_feature_gaps(&self) -> HashMap<String, Vec<String>> {
        let mut gaps = HashMap::new();
        
        for (name, competitor) in &self.competitors {
            let missing_features: Vec<String> = self.our_features
                .iter()
                .filter(|feature| !competitor.features.contains(feature))
                .cloned()
                .collect();
            
            if !missing_features.is_empty() {
                gaps.insert(name.clone(), missing_features);
            }
        }
        
        gaps
    }

    /// Get our unique features
    pub fn get_unique_features(&self) -> Vec<String> {
        let mut all_competitor_features = std::collections::HashSet::new();
        
        for competitor in self.competitors.values() {
            for feature in &competitor.features {
                all_competitor_features.insert(feature);
            }
        }
        
        self.our_features
            .iter()
            .filter(|feature| !all_competitor_features.contains(*feature))
            .cloned()
            .collect()
    }

    /// Get competitive advantage score
    pub fn get_advantage_score(&self) -> f64 {
        let unique_features = self.get_unique_features();
        let total_features = self.our_features.len();
        
        (unique_features.len() as f64 / total_features as f64) * 100.0
    }
}

impl Default for FeatureTracker {
    fn default() -> Self {
        Self::new()
    }
}

