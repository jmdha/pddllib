use criterion::criterion_group;
use pathfinding::directed::bfs::bfs;
use pddllib::{state::State, successor_generation::successors, task::Task};

fn solve(task: &Task) -> Option<Vec<State>> {
    bfs(
        &task.init,
        |state| successors(&task, state),
        |state| state.covers(task, &task.goal),
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

mod miconic {
    use criterion::Criterion;
    use pddllib::translation::translate;

    use super::solve;
    const DOMAIN: &'static str = r#"
(define
	(domain miconic)
	(:requirements :strips)
	(:types
		passenger - object
		floor - object
	)
	(:predicates
		(origin ?person - passenger ?floor - floor)
		(destin ?person - passenger ?floor - floor)
		(above ?floor1 - floor ?floor2 - floor)
		(boarded ?person - passenger)
		(not-boarded ?person - passenger)
		(served ?person - passenger)
		(not-served ?person - passenger)
		(lift-at ?floor - floor)
	)
	(:action board
		:parameters (?f - floor ?p - passenger)
		:precondition 
			(and
				(lift-at ?f)
				(origin ?p ?f)
			)
		:effect 
			(boarded ?p)
	)
	(:action depart
		:parameters (?f - floor ?p - passenger)
		:precondition 
			(and
				(lift-at ?f)
				(destin ?p ?f)
				(boarded ?p)
			)
		:effect 
			(and
				(not (boarded ?p))
				(served ?p)
			)
	)
	(:action up
		:parameters (?f1 - floor ?f2 - floor)
		:precondition 
			(and
				(lift-at ?f1)
				(above ?f1 ?f2)
			)
		:effect 
			(and
				(lift-at ?f2)
				(not (lift-at ?f1))
			)
	)
	(:action down
		:parameters (?f1 - floor ?f2 - floor)
		:precondition 
			(and
				(lift-at ?f1)
				(above ?f2 ?f1)
			)
		:effect 
			(and
				(lift-at ?f2)
				(not (lift-at ?f1))
			)
	)
)
"#;

    const PROBLEM: &'static str = r#"
(define
	(problem mixed-f5-p5-u0-v0-d0-a0-n0-a0-b0-n0-f0)
	(:domain miconic)
	(:objects
		p0 - passenger
		p1 - passenger
		p2 - passenger
		f0 - floor
		f1 - floor
		f2 - floor
		f3 - floor
		f4 - floor
	)
	(:init
		(above f0 f1)
		(above f0 f2)
		(above f0 f3)
		(above f0 f4)
		(above f1 f2)
		(above f1 f3)
		(above f1 f4)
		(above f2 f3)
		(above f2 f4)
		(above f3 f4)
		(origin p0 f0)
		(destin p0 f2)
		(origin p1 f4)
		(destin p1 f0)
		(origin p2 f4)
		(destin p2 f3)
		(lift-at f0)
	)
	(:goal
		(and
			(served p0)
			(served p1)
			(served p2)
		)
	)
)
"#;

    pub fn bench(c: &mut Criterion) {
        let task = translate(DOMAIN, PROBLEM).unwrap();
        c.bench_function("solve miconic", |b| b.iter(|| solve(&task)));
    }
}

mod grid {
    use criterion::Criterion;
    use pddllib::translation::translate;

    use super::solve;
    const DOMAIN: &'static str = r#"
(define
	(domain grid)
	(:requirements :strips)
	(:predicates
		(conn ?x ?y)
		(key-shape ?k ?s)
		(lock-shape ?x ?s)
		(at ?r ?x)
		(at-robot ?x)
		(place ?p)
		(key ?k)
		(shape ?s)
		(locked ?x)
		(holding ?k)
		(open ?x)
		(arm-empty)
	)
	(:action unlock
		:parameters (?curpos ?lockpos ?key ?shape)
		:precondition 
			(and
				(place ?curpos)
				(place ?lockpos)
				(key ?key)
				(shape ?shape)
				(conn ?curpos ?lockpos)
				(key-shape ?key ?shape)
				(lock-shape ?lockpos ?shape)
				(at-robot ?curpos)
				(locked ?lockpos)
				(holding ?key)
			)
		:effect 
			(and
				(open ?lockpos)
				(not (locked ?lockpos))
			)
	)
	(:action move
		:parameters (?curpos ?nextpos)
		:precondition 
			(and
				(place ?curpos)
				(place ?nextpos)
				(at-robot ?curpos)
				(conn ?curpos ?nextpos)
				(open ?nextpos)
			)
		:effect 
			(and
				(at-robot ?nextpos)
				(not (at-robot ?curpos))
			)
	)
	(:action pickup
		:parameters (?curpos ?key)
		:precondition 
			(and
				(place ?curpos)
				(key ?key)
				(at-robot ?curpos)
				(at ?key ?curpos)
				(arm-empty)
			)
		:effect 
			(and
				(holding ?key)
				(not (at ?key ?curpos))
				(not (arm-empty))
			)
	)
	(:action pickup-and-loose
		:parameters (?curpos ?newkey ?oldkey)
		:precondition 
			(and
				(place ?curpos)
				(key ?newkey)
				(key ?oldkey)
				(at-robot ?curpos)
				(holding ?oldkey)
				(at ?newkey ?curpos)
			)
		:effect 
			(and
				(holding ?newkey)
				(at ?oldkey ?curpos)
				(not (holding ?oldkey))
				(not (at ?newkey ?curpos))
			)
	)
	(:action putdown
		:parameters (?curpos ?key)
		:precondition 
			(and
				(place ?curpos)
				(key ?key)
				(at-robot ?curpos)
				(holding ?key)
			)
		:effect 
			(and
				(arm-empty)
				(at ?key ?curpos)
				(not (holding ?key))
			)
	)

)
"#;

    const PROBLEM: &'static str = r#"
(define
	(problem grid-3-3-1-2-1)
	(:domain grid)
	(:objects
		pos0-0
		pos0-1
		pos0-2
		pos1-0
		pos1-1
		pos1-2
		pos2-0
		pos2-1
		pos2-2
		shape0
		key0
		key1
	)
	(:init
		(arm-empty)
		(place pos0-0)
		(place pos0-1)
		(place pos0-2)
		(place pos1-0)
		(place pos1-1)
		(place pos1-2)
		(place pos2-0)
		(place pos2-1)
		(place pos2-2)
		(shape shape0)
		(key key0)
		(key key1)
		(conn pos0-0 pos1-0)
		(conn pos0-0 pos0-1)
		(conn pos0-1 pos1-1)
		(conn pos0-1 pos0-2)
		(conn pos0-1 pos0-0)
		(conn pos0-2 pos1-2)
		(conn pos0-2 pos0-1)
		(conn pos1-0 pos2-0)
		(conn pos1-0 pos1-1)
		(conn pos1-0 pos0-0)
		(conn pos1-1 pos2-1)
		(conn pos1-1 pos1-2)
		(conn pos1-1 pos0-1)
		(conn pos1-1 pos1-0)
		(conn pos1-2 pos2-2)
		(conn pos1-2 pos0-2)
		(conn pos1-2 pos1-1)
		(conn pos2-0 pos2-1)
		(conn pos2-0 pos1-0)
		(conn pos2-1 pos2-2)
		(conn pos2-1 pos1-1)
		(conn pos2-1 pos2-0)
		(conn pos2-2 pos1-2)
		(conn pos2-2 pos2-1) (locked pos0-2)
		(lock-shape pos0-2 shape0)
		(open pos0-0)
		(open pos0-1)
		(open pos1-0)
		(open pos1-1)
		(open pos1-2)
		(open pos2-0)
		(open pos2-1)
		(open pos2-2)
		(key-shape key0 shape0)
		(key-shape key1 shape0)
		(at key0 pos2-1)
		(at key1 pos2-0)
		(at-robot pos0-1)
	)
	(:goal
		(and (at key0 pos2-2))
	)
)
"#;

    pub fn bench(c: &mut Criterion) {
        let task = translate(DOMAIN, PROBLEM).unwrap();
        c.bench_function("solve grid", |b| b.iter(|| solve(&task)));
    }
}

criterion_group!(benches, blocksworld::bench, miconic::bench, grid::bench);
