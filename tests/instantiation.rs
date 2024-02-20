mod blocksworld {
    use pddllib::{
        successor_generation::instantiate_actions, translation::translate,
    };
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
        let operators = instantiate_actions(&task, &task.init);
        assert_eq!(2, operators.len());
        assert_eq!("pickup", &operators[0].action.name);
        assert_eq!("pickup", &operators[1].action.name);
    }
}
