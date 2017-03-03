import Vue from 'vue'
import VueResource from 'vue-resource'

Vue.use(VueResource);

const API_URL = "/api";
const API_EVENTS_URL = API_URL + "/events";

export var app = new Vue({

    el: "#events",

    data: {
        event: {
            name: "",
            description: "",
            date: ""
        },
        newEventState: {
            isOkay: true,
            message: ""
        },
        events: []
    },

    created: function() {
        this.fetchEvents();
    },

    methods: {

        resetEvents: function(newEvents) {
            this.events = newEvents || [];
        },

        fetchEvents: function() {
            this.$http
                .get(API_EVENTS_URL)
                .then((response) => {
                    console.log(response);
                    return response.json();
                }, (response) => {
                    // Error
                    console.log(response)
                })
                .then(this.resetEvents);
        },

        addEvent: function(newEvent) {
            console.log("foo");
            if (newEvent) {
                this.events.push(newEvent);
            }
        },

        fail: function(message) {
            this.newEventState.isOkay = false;
            this.newEventState.message = message;
        },

        failWithResponse: function(response) {
            console.log(response);
            if (response.status == 400) {
                this.fail(response.body.message);
            }
            else {
                this.fail("The server responded with status code " + response.status +
                          ". Please resubmit your event again in couple minutes.");
            }
        },

        submitNewEvent: function() {
            if (this.event.name) {
                var event = {
                    name: this.event.name
                };
                if (this.event.description) {
                    event.description = this.event.description
                }
                if (this.event.date) {
                    event.date = this.event.date
                }
                this.$http
                    .post(API_EVENTS_URL, event)
                    .then((response) => {
                        console.log(response);
                        this.event = {
                            name: "",
                            description: "",
                            date: ""
                        };
                        this.newEventState.isOkay = true;
                        return response.json();
                    }, this.failWithResponse)
                    .then(this.addEvent);
            }
            else {
                this.fail("The event is missing a name. Please provide a name and " +
                          "then resubmit.");
            }
        },

        deleteEvent: function(ix) {
            this.events.splice(ix, 1);
        }
    }
});
