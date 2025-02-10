export type ModalState = ModalStateNone;
export type  ModalType = 'None';

type ModalStateBase = {
    type: ModalType;
}

type ModalStateNone = ModalStateBase & {
    type: 'None'
}

export type ModalAction =
    | { type: "MODAL_CLOSE" };

export const modalReducer = (state: ModalState, action: ModalAction): ModalState => {
    switch (action.type) {
        case "MODAL_CLOSE": {
            return {
                type: 'None'
            }
        }
        default:
            return {
                ...state
            };
    }
};

export const modalInitialState = (): ModalState => ({
    type: "None"
})


