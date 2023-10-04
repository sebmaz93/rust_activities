// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Copy, Clone)]

struct LuggageId(usize);
struct Luggage<State> {
    id: LuggageId,
    state: State,
}

impl<State> Luggage<State> {
    fn next<Next>(self, state: Next) -> Luggage<Next> {
        Luggage { id: self.id, state }
    }
}

struct BeginCustody;
struct CheckIn;
struct OnLoad;
struct OffLoad;
struct AwaitingPickup;
struct EndCustody(LuggageId);

impl Luggage<BeginCustody> {
    fn new(id: LuggageId) -> Self {
        Self {
            id,
            state: BeginCustody,
        }
    }
    fn check_in(self) -> Luggage<CheckIn> {
        self.next(CheckIn)
    }
}

impl Luggage<CheckIn> {
    fn onload(self) -> Luggage<OnLoad> {
        self.next(OnLoad)
    }
}

impl Luggage<OnLoad> {
    fn offload(self) -> Luggage<OffLoad> {
        self.next(OffLoad)
    }
}

impl Luggage<OffLoad> {
    fn carousel(self) -> Luggage<AwaitingPickup> {
        self.next(AwaitingPickup)
    }
}

impl Luggage<AwaitingPickup> {
    fn pickup(self) -> (Luggage<EndCustody>, EndCustody) {
        let id = self.id;
        (self.next(EndCustody(id)), EndCustody(id))
    }
}

fn main() {
    let id = LuggageId(1);
    let luggage = Luggage::new(id);
    let luggage = luggage.check_in().onload().offload().carousel();
    luggage.pickup();
}
