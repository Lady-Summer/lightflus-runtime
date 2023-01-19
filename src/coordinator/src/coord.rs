use proto::common::Dataflow;
use proto::common::DataflowStatus;

use proto::common::Heartbeat;
use proto::common::ResourceId;

use crate::config::CoordinatorConfig;
use crate::managers::Dispatcher;

pub(crate) struct Coordinator {
    dispatcher: Dispatcher,
}

impl Coordinator {
    pub(crate) fn new(config: &CoordinatorConfig) -> Self {
        let dispatcher = Dispatcher::new(config);
        Coordinator { dispatcher }
    }

    pub(crate) async fn create_dataflow(
        &mut self,
        mut dataflow: Dataflow,
    ) -> Result<(), tonic::Status> {
        match dataflow
            .validate()
            .map_err(|err| tonic::Status::invalid_argument(format!("{:?}", err)))
        {
            Ok(_) => {
                let terminate_result = self
                    .terminate_dataflow(dataflow.job_id.as_ref().unwrap())
                    .await;
                if terminate_result.is_err() {
                    return terminate_result.map(|_| ());
                }
                self.dispatcher
                    .create_dataflow(&mut dataflow)
                    .await
                    .map_err(|err| err.to_tonic_status())
            }
            Err(err) => Err(err),
        }
    }

    pub(crate) async fn terminate_dataflow(
        &mut self,
        job_id: &ResourceId,
    ) -> Result<DataflowStatus, tonic::Status> {
        self.dispatcher
            .terminate_dataflow(job_id)
            .await
            .map_err(|err| err.to_tonic_status())
    }

    pub(crate) fn get_dataflow(&self, job_id: &ResourceId) -> Option<Dataflow> {
        self.dispatcher.get_dataflow(job_id)
    }

    pub(crate) fn receive_heartbeart(&mut self, heartbeat: &Heartbeat) {
        self.dispatcher.update_task_manager_heartbeat_status(heartbeat)
    }
}

pub struct CoordinatorException {}
