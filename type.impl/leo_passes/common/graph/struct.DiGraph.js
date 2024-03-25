(function() {var type_impls = {
"leo_passes":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DiGraph%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/leo_passes/common/graph/mod.rs.html#53-143\">source</a><a href=\"#impl-DiGraph%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;N: <a class=\"trait\" href=\"leo_passes/common/graph/trait.Node.html\" title=\"trait leo_passes::common::graph::Node\">Node</a>&gt; <a class=\"struct\" href=\"leo_passes/common/graph/struct.DiGraph.html\" title=\"struct leo_passes::common::graph::DiGraph\">DiGraph</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/leo_passes/common/graph/mod.rs.html#55-57\">source</a><h4 class=\"code-header\">pub fn <a href=\"leo_passes/common/graph/struct.DiGraph.html#tymethod.new\" class=\"fn\">new</a>(nodes: <a class=\"struct\" href=\"https://docs.rs/indexmap/1/indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;N&gt;) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Initializes a new <code>DiGraph</code> from a vector of source nodes.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_edge\" class=\"method\"><a class=\"src rightside\" href=\"src/leo_passes/common/graph/mod.rs.html#60-68\">source</a><h4 class=\"code-header\">pub fn <a href=\"leo_passes/common/graph/struct.DiGraph.html#tymethod.add_edge\" class=\"fn\">add_edge</a>(&amp;mut self, from: N, to: N)</h4></section></summary><div class=\"docblock\"><p>Adds an edge to the graph.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.contains_node\" class=\"method\"><a class=\"src rightside\" href=\"src/leo_passes/common/graph/mod.rs.html#71-73\">source</a><h4 class=\"code-header\">pub fn <a href=\"leo_passes/common/graph/struct.DiGraph.html#tymethod.contains_node\" class=\"fn\">contains_node</a>(&amp;self, node: N) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Returns <code>true</code> if the graph contains the given node.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.post_order\" class=\"method\"><a class=\"src rightside\" href=\"src/leo_passes/common/graph/mod.rs.html#77-108\">source</a><h4 class=\"code-header\">pub fn <a href=\"leo_passes/common/graph/struct.DiGraph.html#tymethod.post_order\" class=\"fn\">post_order</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://docs.rs/indexmap/1/indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;N&gt;, <a class=\"enum\" href=\"leo_passes/common/graph/enum.DiGraphError.html\" title=\"enum leo_passes::common::graph::DiGraphError\">DiGraphError</a>&lt;N&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Returns the post-order ordering of the graph.\nDetects if there is a cycle in the graph.</p>\n</div></details><section id=\"method.contains_cycle_from\" class=\"method\"><a class=\"src rightside\" href=\"src/leo_passes/common/graph/mod.rs.html#114-142\">source</a><h4 class=\"code-header\">fn <a href=\"leo_passes/common/graph/struct.DiGraph.html#tymethod.contains_cycle_from\" class=\"fn\">contains_cycle_from</a>(\n    &amp;self,\n    node: N,\n    discovered: &amp;mut <a class=\"struct\" href=\"https://docs.rs/indexmap/1/indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;N&gt;,\n    finished: &amp;mut <a class=\"struct\" href=\"https://docs.rs/indexmap/1/indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;N&gt;\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;N&gt;</h4></section></div></details>",0,"leo_passes::common::graph::StructGraph","leo_passes::common::graph::CallGraph","leo_passes::common::graph::ImportGraph"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-DiGraph%3CN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/leo_passes/common/graph/mod.rs.html#44\">source</a><a href=\"#impl-Debug-for-DiGraph%3CN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;N: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"leo_passes/common/graph/trait.Node.html\" title=\"trait leo_passes::common::graph::Node\">Node</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"leo_passes/common/graph/struct.DiGraph.html\" title=\"struct leo_passes::common::graph::DiGraph\">DiGraph</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/leo_passes/common/graph/mod.rs.html#44\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.77.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","leo_passes::common::graph::StructGraph","leo_passes::common::graph::CallGraph","leo_passes::common::graph::ImportGraph"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()