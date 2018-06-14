import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://sweetbits.com.au/rra/login.php')

WebUI.setText(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/input_email'), 'pl250401@gmail.com')

WebUI.setText(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/input_password'), 'test')

WebUI.click(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/input_login'))

WebUI.click(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/a_Support Centre'))

WebUI.click(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/a_Submit New Enquiry'))

WebUI.setText(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/input_subject'), 'sub030501')

WebUI.selectOptionByValue(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/select_Select'), 'High', 
    true)

WebUI.click(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/body_form-control wysihtml5-ed'))

WebUI.setText(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/body_form-control wysihtml5-ed'), 
    'test')

WebUI.click(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/input_submit'))

WebUI.verifyElementPresent(findTestObject('New Folder/SubmitInquiry/Page_Vip Recover/Page_Vip Recover/a_View Enquiries'), 
    0)

WebUI.closeBrowser()

