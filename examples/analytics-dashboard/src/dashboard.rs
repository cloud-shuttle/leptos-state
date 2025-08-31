use leptos::*;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use rand::Rng;

// Data Models
#[derive(Debug, Clone, PartialEq)]
pub struct Metric {
    pub id: Uuid,
    pub name: String,
    pub value: f64,
    pub change: f64,
    pub trend: Trend,
    pub category: MetricCategory,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Trend {
    Up,
    Down,
    Stable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricCategory {
    Revenue,
    Users,
    Performance,
    Engagement,
}

// Mock Data Generator
fn generate_mock_data() -> Vec<Metric> {
    let now = Utc::now();
    
    vec![
        Metric {
            id: Uuid::new_v4(),
            name: "Total Revenue".to_string(),
            value: 125000.0,
            change: 12.5,
            trend: Trend::Up,
            category: MetricCategory::Revenue,
            timestamp: now,
        },
        Metric {
            id: Uuid::new_v4(),
            name: "Active Users".to_string(),
            value: 15420.0,
            change: -2.1,
            trend: Trend::Down,
            category: MetricCategory::Users,
            timestamp: now,
        },
        Metric {
            id: Uuid::new_v4(),
            name: "Page Load Time".to_string(),
            value: 1.2,
            change: -15.3,
            trend: Trend::Up,
            category: MetricCategory::Performance,
            timestamp: now,
        },
        Metric {
            id: Uuid::new_v4(),
            name: "Session Duration".to_string(),
            value: 8.5,
            change: 5.7,
            trend: Trend::Up,
            category: MetricCategory::Engagement,
            timestamp: now,
        },
    ]
}

// Main Dashboard Component
#[component]
pub fn AnalyticsDashboard() -> impl IntoView {
    let (metrics, set_metrics) = create_signal(generate_mock_data());
    let (selected_timeframe, set_selected_timeframe) = create_signal("7d".to_string());
    let (is_loading, set_is_loading) = create_signal(false);

    // Refresh data function
    let refresh_data = move |_| {
        set_is_loading.set(true);
        set_timeout(move || {
            set_metrics.set(generate_mock_data());
            set_is_loading.set(false);
        }, std::time::Duration::from_millis(1000));
    };

    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; padding: 2rem;">
            
            <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border-radius: 1rem; padding: 2rem; margin-bottom: 2rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);">
                <h1 style="font-size: 2.5rem; font-weight: 700; color: #1e293b; margin-bottom: 0.5rem;">Analytics Dashboard</h1>
                <p style="color: #64748b; font-size: 1.1rem;">Real-time business metrics and insights</p>
                
                <div style="display: flex; gap: 1rem; margin-top: 1rem; align-items: center;">
                    <div style="display: flex; background: #f1f5f9; border-radius: 0.5rem; padding: 0.25rem;">
                        <button
                            style=move || if selected_timeframe.get() == "1d" { "padding: 0.5rem 1rem; border: none; background: white; border-radius: 0.375rem; cursor: pointer; transition: all 0.2s; font-weight: 500; color: #3b82f6; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);" } else { "padding: 0.5rem 1rem; border: none; background: transparent; border-radius: 0.375rem; cursor: pointer; transition: all 0.2s; font-weight: 500;" }
                            on:click=move |_| set_selected_timeframe.set("1d".to_string())
                        >
                            "1D"
                        </button>
                        <button
                            style=move || if selected_timeframe.get() == "7d" { "padding: 0.5rem 1rem; border: none; background: white; border-radius: 0.375rem; cursor: pointer; transition: all 0.2s; font-weight: 500; color: #3b82f6; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);" } else { "padding: 0.5rem 1rem; border: none; background: transparent; border-radius: 0.375rem; cursor: pointer; transition: all 0.2s; font-weight: 500;" }
                            on:click=move |_| set_selected_timeframe.set("7d".to_string())
                        >
                            "7D"
                        </button>
                        <button
                            style=move || if selected_timeframe.get() == "30d" { "padding: 0.5rem 1rem; border: none; background: white; border-radius: 0.375rem; cursor: pointer; transition: all 0.2s; font-weight: 500; color: #3b82f6; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);" } else { "padding: 0.5rem 1rem; border: none; background: transparent; border-radius: 0.375rem; cursor: pointer; transition: all 0.2s; font-weight: 500;" }
                            on:click=move |_| set_selected_timeframe.set("30d".to_string())
                        >
                            "30D"
                        </button>
                    </div>
                    
                    <button 
                        style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 0.5rem; cursor: pointer; transition: background 0.2s;" 
                        on:click=refresh_data
                    >
                        {move || if is_loading.get() { "Refreshing..." } else { "Refresh" }}
                    </button>
                </div>
            </div>
            
            {move || if is_loading.get() {
                view! {
                    <div style="display: flex; align-items: center; justify-content: center; height: 200px; color: #64748b;">
                        <div style="width: 20px; height: 20px; border: 2px solid #e2e8f0; border-top: 2px solid #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin-right: 0.5rem;"></div>
                        "Updating data..."
                    </div>
                }.into_view()
            } else {
                view! {
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem;">
                        {move || metrics.get().into_iter().map(|metric| {
                            let category_style = match metric.category {
                                MetricCategory::Revenue => "background: #dbeafe; color: #1e40af;",
                                MetricCategory::Users => "background: #dcfce7; color: #166534;",
                                MetricCategory::Performance => "background: #fef3c7; color: #d97706;",
                                MetricCategory::Engagement => "background: #f3e8ff; color: #7c3aed;",
                            };
                            
                            let change_color = match metric.trend {
                                Trend::Up => "#10b981",
                                Trend::Down => "#ef4444",
                                Trend::Stable => "#6b7280",
                            };
                            
                            let change_icon = match metric.trend {
                                Trend::Up => "↗",
                                Trend::Down => "↘",
                                Trend::Stable => "→",
                            };
                            
                            let formatted_value = if metric.value >= 1000.0 {
                                format!("${:.1}K", metric.value / 1000.0)
                            } else {
                                format!("{:.1}", metric.value)
                            };
                            
                            view! {
                                <div style="background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border-radius: 1rem; padding: 1.5rem; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1); transition: transform 0.2s;">
                                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                                        <span style="font-size: 0.875rem; color: #64748b; font-weight: 500; text-transform: uppercase; letter-spacing: 0.05em;">{metric.name}</span>
                                        <span style=format!("padding: 0.25rem 0.5rem; border-radius: 0.25rem; font-size: 0.75rem; font-weight: 600; {}", category_style)>
                                            {match metric.category {
                                                MetricCategory::Revenue => "Revenue",
                                                MetricCategory::Users => "Users",
                                                MetricCategory::Performance => "Performance",
                                                MetricCategory::Engagement => "Engagement",
                                            }}
                                        </span>
                                    </div>
                                    <div style="font-size: 2rem; font-weight: 700; color: #1e293b; margin-bottom: 0.5rem;">{formatted_value}</div>
                                    <div style=format!("display: flex; align-items: center; gap: 0.5rem; font-size: 0.875rem; font-weight: 500; color: {}", change_color)>
                                        <span>{change_icon}</span>
                                        <span>{format!("{:.1}%", metric.change.abs())}</span>
                                        <span>"from last period"</span>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_view()
            }}
        </div>
    }
}
