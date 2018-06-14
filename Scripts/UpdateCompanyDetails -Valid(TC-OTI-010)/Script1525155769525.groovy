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

WebUI.setText(findTestObject('UpdateCompanyDetails/Page_Vip Recover/input_email'), 'pl250401@gmail.com')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('UpdateCompanyDetails/Page_Vip Recover/input_password'), 'test')

WebUI.click(findTestObject('UpdateCompanyDetails/Page_Vip Recover/input_login'))

WebUI.delay(5)

WebUI.click(findTestObject('New Folder/Debug/Page_Vip Recover/a_Update company details'))

WebUI.clearText(findTestObject('UpdateCompanyDetails/Page_Vip Recover/input_phone'))

WebUI.setText(findTestObject('UpdateCompanyDetails/Page_Vip Recover/input_phone'), '3134567890')

WebUI.setText(findTestObject('UpdateCompanyDetails/Page_Vip Recover/input_password_1'), 'test')

WebUI.setText(findTestObject('UpdateCompanyDetails/Page_Vip Recover/input_con_password'), 'test')

WebUI.click(findTestObject('UpdateCompanyDetails/Page_Vip Recover/input_submit'))

x = WebUI.getText(findTestObject('UpdateCompanyDetails/Page_Vip Recover/div_Account updated successful'))

WebUI.verifyMatch(x, 'Account updated successfully.', false)

WebUI.closeBrowser()

