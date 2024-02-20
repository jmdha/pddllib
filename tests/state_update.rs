mod blocksworld {
    use pddllib::translation::translate;
    pub const DOMAIN: &'static str = "
;; source: https://github.com/AI-Planning/pddl-generators/blob/main/blocksworld/domain.pddl
;;
(define (domain blocksworld)

(:requirements :strips)

(:predicates (clear ?x)
             (on-table ?x)
             (arm-empty)
             (holding ?x)
             (on ?x ?y))

(:action pickup
  :parameters (?ob)
  :precondition (and (clear ?ob) (on-table ?ob) (arm-empty))
  :effect (and (holding ?ob) (not (clear ?ob)) (not (on-table ?ob)) 
               (not (arm-empty))))

(:action putdown
  :parameters  (?ob)
  :precondition (holding ?ob)
  :effect (and (clear ?ob) (arm-empty) (on-table ?ob) 
               (not (holding ?ob))))

(:action stack
  :parameters  (?ob ?underob)
  :precondition (and (clear ?underob) (holding ?ob))
  :effect (and (arm-empty) (clear ?ob) (on ?ob ?underob)
               (not (clear ?underob)) (not (holding ?ob))))

(:action unstack
  :parameters  (?ob ?underob)
  :precondition (and (on ?ob ?underob) (clear ?ob) (arm-empty))
  :effect (and (holding ?ob) (clear ?underob)
               (not (on ?ob ?underob)) (not (clear ?ob)) (not (arm-empty)))))
";
    pub const PROBLEM: &'static str = "
    ;; base case
;;
(define (problem blocksworld-01)
 (:domain blocksworld)
 (:objects  b1 b2 - object)
 (:init 
    (arm-empty)
    (clear b2)
    (on-table b2)
    (clear b1)
    (on-table b1)
)
 (:goal (and 
    (clear b1)
    (on b1 b2)
    (on-table b2)
)))
";
    #[test]
    fn instantiate() {
        let task = translate(DOMAIN, PROBLEM).unwrap();
        assert!(task.init.has_unary(0, &0));
        assert!(task.init.has_unary(1, &0));
        assert!(task.init.has_nullary(2));
        let state = task.init.apply(&task.actions[0], &vec![0]);
        assert!(state.has_unary(3, &0));
        assert!(!state.has_unary(0, &0));
        assert!(!state.has_unary(1, &0));
        assert!(!state.has_nullary(2));
    }
}
