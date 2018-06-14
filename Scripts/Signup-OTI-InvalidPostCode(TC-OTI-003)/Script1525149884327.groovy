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

WebUI.openBrowser('')

WebUI.navigateToUrl('http://sweetbits.com.au/rra/login.php')

WebUI.maximizeWindow()

WebUI.click(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/a_Register as a Contributor'))

WebUI.click(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/a_Register as a Refrigerant Re'))

WebUI.click(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/a_If you are a one-time import'))

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_abn'), '56096929574')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_company'), 'cmp250401')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_contact_person'), 'cmp cnt person')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_phone'), '1234567890')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_email'), Email)

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_mobile'), '1234567890')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_fax'), '1234567890')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_street'), 'Line 1')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_street2'), 'Line2')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_suburb'), 'Mumbai')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_postcode'), '987456')

WebUI.click(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/button_Select'))

WebUI.delay(5)

WebUI.selectOptionByValue(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/select_Select        Afghanist'), 'India', 
    true)

WebUI.delay(15)

WebUI.click(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/btn_State'))

WebUI.delay(4)

WebUI.selectOptionByValue(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/select_SelectAndaman and Nicob'), 'Maharashtra', 
    true)

WebUI.delay(15)

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_password'), 'test')

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_con_password'), 'test')

WebUI.click(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/label_Motor vehicles'))

WebUI.click(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_terms'))

WebUI.setText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_captchtext'), '53232')

WebUI.click(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/input_register'))

PostalCodeError = WebUI.getText(findTestObject('New Folder/Signup-OTI/Page_Vip Recover/label_Please enter a valid ema'))

WebUI.verifyMatch(PostalCodeError, 'Please enter correct pincode.', false)

WebUI.closeBrowser()

