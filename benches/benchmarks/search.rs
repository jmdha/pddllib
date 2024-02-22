use criterion::criterion_group;
use pathfinding::directed::bfs::bfs;
use pddllib::{state::State, successor_generation::successors, task::Task};

fn solve(task: &Task) -> Option<Vec<State>> {
    bfs(
        &task.init,
        |state| successors(&task, state),
        |state| state.covers(&task.goal),
    )
}

mod blocksworld {
    use criterion::Criterion;
    use pddllib::translation::translate;

    use super::solve;
    const DOMAIN: &'static str = r#"
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
"#;

    const PROBLEM: &'static str = r#"
;; blocks=5, out_folder=testing/easy, instance_id=1, seed=1007

(define (problem blocksworld-01)
 (:domain blocksworld)
 (:objects b1 b2 b3 b4 b5 - object)
 (:init 
    (arm-empty)
    (clear b3)
    (on b3 b5)
    (on b5 b4)
    (on-table b4)
    (clear b2)
    (on b2 b1)
    (on-table b1))
 (:goal  (and 
    (clear b4)
    (on b4 b3)
    (on-table b3)
    (clear b2)
    (on-table b2)
    (clear b1)
    (on b1 b5)
    (on-table b5))))
"#;

    pub fn bench(c: &mut Criterion) {
        let task = translate(DOMAIN, PROBLEM).unwrap();
        c.bench_function("solve blocksworld", |b| b.iter(|| solve(&task)));
    }
}

mod rovers {
    use criterion::Criterion;
    use pddllib::translation::translate;

    use super::solve;
    const DOMAIN: &'static str = r#"
;; source: https://github.com/AI-Planning/pddl-generators/blob/main/rovers/domain.pddl
;; updates:
;;   - since actions are performed sequentially and immediately
;;     'channel_free' and 'available' predicates are removed
;;   -

(define (domain rover)
(:requirements :strips :typing)
(:types rover waypoint store camera mode lander objective)

(:predicates
    (at ?x - rover ?y - waypoint)
    (at_lander ?x - lander ?y - waypoint)
    (can_traverse ?r - rover ?x - waypoint ?y - waypoint)
	(equipped_for_soil_analysis ?r - rover)
    (equipped_for_rock_analysis ?r - rover)
    (equipped_for_imaging ?r - rover)
    (empty ?s - store)
    (have_rock_analysis ?r - rover ?w - waypoint)
    (have_soil_analysis ?r - rover ?w - waypoint)
    (full ?s - store)
	(calibrated ?c - camera ?r - rover)
	(supports ?c - camera ?m - mode)
    (visible ?w - waypoint ?p - waypoint)
    (have_image ?r - rover ?o - objective ?m - mode)
    (communicated_soil_data ?w - waypoint)
    (communicated_rock_data ?w - waypoint)
    (communicated_image_data ?o - objective ?m - mode)
	(at_soil_sample ?w - waypoint)
	(at_rock_sample ?w - waypoint)
    (visible_from ?o - objective ?w - waypoint)
	(store_of ?s - store ?r - rover)
	(calibration_target ?i - camera ?o - objective)
	(on_board ?i - camera ?r - rover)
)

(:action navigate
:parameters (?x - rover ?y - waypoint ?z - waypoint)
:precondition (and
    (can_traverse ?x ?y ?z)
    (at ?x ?y)
    (visible ?y ?z))
:effect (and
    (not (at ?x ?y))
    (at ?x ?z))
)

(:action sample_soil
:parameters (?x - rover ?s - store ?p - waypoint)
:precondition (and
    (at ?x ?p)
    (at_soil_sample ?p)
	(equipped_for_soil_analysis ?x)
	(store_of ?s ?x)
	(empty ?s))
:effect (and
    (not (empty ?s))
    (full ?s)
    (have_soil_analysis ?x ?p)
    (not (at_soil_sample ?p)))
)

(:action sample_rock
:parameters (?x - rover ?s - store ?p - waypoint)
:precondition (and
    (at ?x ?p)
    (at_rock_sample ?p)
	(equipped_for_rock_analysis ?x)
	(store_of ?s ?x)
	(empty ?s))
:effect (and
    (not (empty ?s))
    (full ?s)
    (have_rock_analysis ?x ?p)
	(not (at_rock_sample ?p)))
)

(:action drop
:parameters (?x - rover ?y - store)
:precondition (and
    (store_of ?y ?x)
    (full ?y))
:effect (and
    (not (full ?y))
    (empty ?y))
)

(:action calibrate
 :parameters (?r - rover ?i - camera ?t - objective ?w - waypoint)
 :precondition (and
    (equipped_for_imaging ?r)
    (calibration_target ?i ?t)
    (at ?r ?w)
    (visible_from ?t ?w)
    (on_board ?i ?r))
 :effect (and
    (calibrated ?i ?r))
)

(:action take_image
 :parameters (?r - rover ?p - waypoint ?o - objective ?i - camera ?m - mode)
 :precondition (and
    (calibrated ?i ?r)
    (on_board ?i ?r)
    (equipped_for_imaging ?r)
    (supports ?i ?m)
    (visible_from ?o ?p)
    (at ?r ?p))
 :effect (and
    (have_image ?r ?o ?m)
    (not (calibrated ?i ?r)))
)

(:action communicate_soil_data
 :parameters (?r - rover ?l - lander ?p - waypoint ?x - waypoint ?y - waypoint)
 :precondition (and
    (at ?r ?x)
    (at_lander ?l ?y)
    (have_soil_analysis ?r ?p)
    (visible ?x ?y))
 :effect (and
    (communicated_soil_data ?p))
)

(:action communicate_rock_data
 :parameters (?r - rover ?l - lander ?p - waypoint ?x - waypoint ?y - waypoint)
 :precondition (and
    (at ?r ?x)
    (at_lander ?l ?y)
    (have_rock_analysis ?r ?p)
    (visible ?x ?y))
 :effect (and
    (communicated_rock_data ?p))
)

(:action communicate_image_data
 :parameters (?r - rover ?l - lander ?o - objective ?m - mode
	      ?x - waypoint ?y - waypoint)
 :precondition (and
    (at ?r ?x)
    (at_lander ?l ?y)
    (have_image ?r ?o ?m)
    (visible ?x ?y))
 :effect (and
    (communicated_image_data ?o ?m)))
)
"#;

    const PROBLEM: &'static str = r#"
;; rovers=1, waypoints=4, cameras=1, objectives=1, out_folder=testing/easy, instance_id=1, seed=1007

(define (problem rover-01)
 (:domain rover)
 (:objects 
    general - lander
    colour high_res low_res - mode
    rover1 - rover
    rover1store - store
    waypoint1 waypoint2 waypoint3 waypoint4 - waypoint
    camera1 - camera
    objective1 - objective)
 (:init 
    (at_lander general waypoint1)
    (at rover1 waypoint2)
    (equipped_for_soil_analysis rover1)
    (equipped_for_rock_analysis rover1)
    (equipped_for_imaging rover1)
    (empty rover1store)
    (store_of rover1store rover1)
    (at_rock_sample waypoint1)
    (at_rock_sample waypoint2)
    (at_rock_sample waypoint3)
    (at_soil_sample waypoint2)
    (at_soil_sample waypoint3)
    (visible waypoint3 waypoint4)
    (visible waypoint4 waypoint3)
    (visible waypoint1 waypoint4)
    (visible waypoint2 waypoint3)
    (visible waypoint3 waypoint2)
    (visible waypoint4 waypoint1)
    (visible waypoint1 waypoint2)
    (visible waypoint2 waypoint1)
    (visible waypoint2 waypoint4)
    (visible waypoint4 waypoint2)
    (visible waypoint1 waypoint3)
    (visible waypoint3 waypoint1)
    (can_traverse rover1 waypoint3 waypoint4)
    (can_traverse rover1 waypoint4 waypoint3)
    (can_traverse rover1 waypoint1 waypoint4)
    (can_traverse rover1 waypoint2 waypoint3)
    (can_traverse rover1 waypoint3 waypoint2)
    (can_traverse rover1 waypoint4 waypoint1)
    (can_traverse rover1 waypoint2 waypoint4)
    (can_traverse rover1 waypoint4 waypoint2)
    (calibration_target camera1 objective1)
    (on_board camera1 rover1)
    (supports camera1 high_res)
    (supports camera1 colour)
    (supports camera1 low_res)
    (visible_from objective1 waypoint1)
    (visible_from objective1 waypoint3)
    (visible_from objective1 waypoint2)
    (visible_from objective1 waypoint4))
 (:goal  (and 
    (communicated_rock_data waypoint3)
    (communicated_rock_data waypoint2)
    (communicated_soil_data waypoint2)
    )))
"#;

    pub fn bench(c: &mut Criterion) {
        let task = translate(DOMAIN, PROBLEM).unwrap();
        c.bench_function("solve rovers", |b| b.iter(|| solve(&task)));
    }
}

criterion_group!(benches, blocksworld::bench, rovers::bench);
