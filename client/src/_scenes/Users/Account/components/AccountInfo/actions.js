import { userConstants } from '~/_constants';

/*
* enableEdit
*
* Enable edit of form
*/
const enableEditableForm = () => ({
	type: userConstants.ACCOUNT_ENABLE_EDITABLE_FORM
});

const accountInfoActions = {
	enableEditableForm
}

export default accountInfoActions;