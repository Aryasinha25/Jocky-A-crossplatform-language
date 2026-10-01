use jocky_lexer::lex;
use jocky_parser::parse;
use jocky_ir::generate_ir;
use jocky_runtime::Runtime;

#[test]
fn test_end_to_end_investigation() {
    let source = "TARGET \"LOCAL\"\nCOLLECT PROCESS\nCORRELATE PROCESS WITH NETWORK\nDETECT SUSPICIOUS_PROCESS\nSCORE FINDINGS\nGENERATE REPORT";
    
    // 1. Lex
    let tokens = lex(source).unwrap();
    assert!(!tokens.is_empty());
    
    // 2. Parse
    let ast = parse(&tokens).unwrap();
    assert_eq!(ast.len(), 6);
    
    // 3. IR
    let ir = generate_ir(&ast);
    
    // 4. Runtime
    let mut runtime = Runtime::new();
    let result = runtime.execute(&ir);
    assert!(result.is_ok());
    
    // 5. Evidence
    let evidence = runtime.get_evidence();
    // Since we called COLLECT PROCESS, and sysinfo collects real processes:
    assert!(evidence.len() > 0);
    
    // Check if at least one process was collected
    let has_process = evidence.iter().any(|e| e.evidence_type == "process");
    assert!(has_process);
    
    let result = runtime.generate_result();
    assert_eq!(result.target, "LOCAL");
}
